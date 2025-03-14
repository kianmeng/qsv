static USAGE: &str = "
Sorts CSV data lexicographically.

Note that this requires reading all of the CSV data into memory. If
you need to sort a large file that may not fit into memory, use the
extsort command instead.

Usage:
    qsv sort [options] [<input>]

sort options:
    -s, --select <arg>     Select a subset of columns to sort.
                           See 'qsv select --help' for the format details.
    -N, --numeric          Compare according to string numerical value
    -R, --reverse          Reverse order
    --random               Random order
    --seed <number>        RNG seed
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Namely, it will be sorted with the rest
                           of the rows. Otherwise, the first row will always
                           appear as the header row in the output.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -u, --uniq             When set, identical consecutive lines will be dropped
                           to keep only one line per sorted value.
";

use self::Number::{Float, Int};
use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::util;
use crate::CliResult;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use rayon::prelude::*;
use serde::Deserialize;
use std::cmp;

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_select: SelectColumns,
    flag_numeric: bool,
    flag_reverse: bool,
    flag_random: bool,
    flag_seed: Option<u64>,
    flag_jobs: Option<usize>,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_uniq: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let numeric = args.flag_numeric;
    let reverse = args.flag_reverse;
    let random = args.flag_random;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    util::njobs(args.flag_jobs);

    // Seeding rng
    let seed = args.flag_seed;

    let mut all = rdr.byte_records().collect::<Result<Vec<_>, _>>()?;
    match (numeric, reverse, random) {
        (_, _, true) => {
            // we don't need cryptographically strong RNGs for this
            if let Some(val) = seed {
                let mut rng = StdRng::seed_from_u64(val); //DevSkim: ignore DS148264
                SliceRandom::shuffle(&mut *all, &mut rng);
            } else {
                let mut rng = ::rand::thread_rng();
                SliceRandom::shuffle(&mut *all, &mut rng);
            }
        }
        (false, false, false) => all.par_sort_unstable_by(|r1, r2| {
            let a = sel.select(r1);
            let b = sel.select(r2);
            iter_cmp(a, b)
        }),
        (true, false, false) => all.par_sort_unstable_by(|r1, r2| {
            let a = sel.select(r1);
            let b = sel.select(r2);
            iter_cmp_num(a, b)
        }),
        (false, true, false) => all.par_sort_unstable_by(|r1, r2| {
            let a = sel.select(r1);
            let b = sel.select(r2);
            iter_cmp(b, a)
        }),
        (true, true, false) => all.par_sort_unstable_by(|r1, r2| {
            let a = sel.select(r1);
            let b = sel.select(r2);
            iter_cmp_num(b, a)
        }),
    }

    let mut wtr = Config::new(&args.flag_output).writer()?;
    let mut prev: Option<csv::ByteRecord> = None;
    rconfig.write_headers(&mut rdr, &mut wtr)?;
    for r in all {
        if args.flag_uniq {
            match prev {
                Some(other_r) => match iter_cmp(sel.select(&r), sel.select(&other_r)) {
                    cmp::Ordering::Equal => (),
                    _ => {
                        wtr.write_byte_record(&r)?;
                    }
                },
                None => {
                    wtr.write_byte_record(&r)?;
                }
            }

            prev = Some(r);
        } else {
            wtr.write_byte_record(&r)?;
        }
    }
    Ok(wtr.flush()?)
}

/// Order `a` and `b` lexicographically using `Ord`
#[inline]
pub fn iter_cmp<A, L, R>(mut a: L, mut b: R) -> cmp::Ordering
where
    A: Ord,
    L: Iterator<Item = A>,
    R: Iterator<Item = A>,
{
    loop {
        match (a.next(), b.next()) {
            (None, None) => return cmp::Ordering::Equal,
            (None, _) => return cmp::Ordering::Less,
            (_, None) => return cmp::Ordering::Greater,
            (Some(x), Some(y)) => match x.cmp(&y) {
                cmp::Ordering::Equal => (),
                non_eq => return non_eq,
            },
        }
    }
}

/// Try parsing `a` and `b` as numbers when ordering
#[inline]
pub fn iter_cmp_num<'a, L, R>(mut a: L, mut b: R) -> cmp::Ordering
where
    L: Iterator<Item = &'a [u8]>,
    R: Iterator<Item = &'a [u8]>,
{
    loop {
        match (next_num(&mut a), next_num(&mut b)) {
            (None, None) => return cmp::Ordering::Equal,
            (None, _) => return cmp::Ordering::Less,
            (_, None) => return cmp::Ordering::Greater,
            (Some(x), Some(y)) => match compare_num(x, y) {
                cmp::Ordering::Equal => (),
                non_eq => return non_eq,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Number {
    Int(i64),
    Float(f64),
}

#[inline]
fn compare_num(n1: Number, n2: Number) -> cmp::Ordering {
    match (n1, n2) {
        (Int(i1), Int(i2)) => i1.cmp(&i2),
        #[allow(clippy::cast_precision_loss)]
        (Int(i1), Float(f2)) => compare_float(i1 as f64, f2),
        #[allow(clippy::cast_precision_loss)]
        (Float(f1), Int(i2)) => compare_float(f1, i2 as f64),
        (Float(f1), Float(f2)) => compare_float(f1, f2),
    }
}

#[inline]
fn compare_float(f1: f64, f2: f64) -> cmp::Ordering {
    f1.partial_cmp(&f2).unwrap_or(cmp::Ordering::Equal)
}

#[inline]
fn next_num<'a, X>(xs: &mut X) -> Option<Number>
where
    X: Iterator<Item = &'a [u8]>,
{
    xs.next()
        .map(|bytes| unsafe { std::str::from_utf8_unchecked(bytes) })
        .and_then(|s| {
            if let Ok(i) = s.parse::<i64>() {
                Some(Number::Int(i))
            } else if let Ok(f) = s.parse::<f64>() {
                Some(Number::Float(f))
            } else {
                None
            }
        })
}
