static USAGE: &str = r#"
Deduplicates CSV rows. 

Note that this requires reading all of the CSV data into memory because because the 
rows need to be sorted first. 

That is, unless the --sorted option is used to indicate the CSV is already sorted
(typically, with the extsort command). This will make dedup run in streaming mode 
with constant memory.

Either way, the output will not only be deduplicated, it will also be sorted.

A duplicate count will also be sent to <stderr>.

Usage:
    qsv dedup [options] [<input>]

sort options:
    -s, --select <arg>         Select a subset of columns to dedup.
                               Note that the outputs will remain at the full width
                               of the CSV.
                               See 'qsv select --help' for the format details.
    -C, --no-case              Compare strings disregarding case 
    --sorted                   The input is already sorted. Do not load the CSV into
                               memory to sort it first. Meant to be used in tandem and
                               after an extsort.
    -D, --dupes-output <file>  Write duplicates to <file>.
    -H, --human-readable       Comma separate duplicate count.
    -j, --jobs <arg>           The number of jobs to run in parallel when sorting
                               an unsorted CSV, before deduping.
                               When not set, the number of jobs is set to the
                               number of CPUs detected.
                               Does not work with --sorted option as its not
                               multithreaded.

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. That is, it will be sorted with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. (default: ,)
"#;

use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::util;
use crate::CliResult;
use csv::ByteRecord;
use rayon::prelude::*;
use serde::Deserialize;
use std::cmp;

use crate::cmd::sort::iter_cmp;
#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_select: SelectColumns,
    flag_no_case: bool,
    flag_sorted: bool,
    flag_dupes_output: Option<String>,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_human_readable: bool,
    flag_jobs: Option<usize>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let no_case = args.flag_no_case;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let dupes_output = args.flag_dupes_output.is_some();
    let mut dupewtr = Config::new(&args.flag_dupes_output).writer()?;

    let headers = rdr.byte_headers()?.clone();
    if dupes_output {
        dupewtr.write_byte_record(&headers)?;
    }
    let sel = rconfig.selection(&headers)?;

    rconfig.write_headers(&mut rdr, &mut wtr)?;
    let mut dupe_count = 0_usize;

    if args.flag_sorted {
        let mut record = ByteRecord::new();
        let mut next_record = ByteRecord::new();

        rdr.read_byte_record(&mut record)?;
        loop {
            let more_records = rdr.read_byte_record(&mut next_record)?;
            if !more_records {
                wtr.write_byte_record(&record)?;
                break;
            };
            let a = sel.select(&record);
            let b = sel.select(&next_record);
            let comparison = if no_case {
                iter_cmp_no_case(a, b)
            } else {
                iter_cmp(a, b)
            };
            match comparison {
                cmp::Ordering::Equal => {
                    dupe_count += 1;
                    if dupes_output {
                        dupewtr.write_byte_record(&record)?;
                    }
                }
                cmp::Ordering::Less => {
                    wtr.write_byte_record(&record)?;
                    record.clone_from(&next_record);
                }
                cmp::Ordering::Greater => {
                    return fail_format!(
                        "Aborting! Input not sorted! {record:?} is greater than {next_record:?}"
                    );
                }
            }
        }
    } else {
        util::njobs(args.flag_jobs);

        let mut all = rdr.byte_records().collect::<Result<Vec<_>, _>>()?;
        all.par_sort_unstable_by(|r1, r2| {
            let a = sel.select(r1);
            let b = sel.select(r2);
            iter_cmp(a, b)
        });

        let mut current = 0;
        while current + 1 < all.len() {
            let a = sel.select(&all[current]);
            let b = sel.select(&all[current + 1]);
            if no_case {
                if iter_cmp_no_case(a, b) == cmp::Ordering::Equal {
                    dupe_count += 1;
                    if dupes_output {
                        dupewtr.write_byte_record(&all[current])?;
                    }
                } else {
                    wtr.write_byte_record(&all[current])?;
                }
            } else if iter_cmp(a, b) == cmp::Ordering::Equal {
                dupe_count += 1;
                if dupes_output {
                    dupewtr.write_byte_record(&all[current])?;
                }
            } else {
                wtr.write_byte_record(&all[current])?;
            }
            current += 1;
        }
        wtr.write_byte_record(&all[current])?;
    }

    dupewtr.flush()?;

    if args.flag_human_readable {
        use thousands::Separable;

        eprintln!("{}", dupe_count.separate_with_commas());
    } else {
        eprintln!("{dupe_count}");
    }

    Ok(wtr.flush()?)
}

/// Try comparing `a` and `b` ignoring the case
#[inline]
pub fn iter_cmp_no_case<'a, L, R>(mut a: L, mut b: R) -> cmp::Ordering
where
    L: Iterator<Item = &'a [u8]>,
    R: Iterator<Item = &'a [u8]>,
{
    loop {
        match (next_no_case(&mut a), next_no_case(&mut b)) {
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

#[inline]
fn next_no_case<'a, X>(xs: &mut X) -> Option<String>
where
    X: Iterator<Item = &'a [u8]>,
{
    xs.next()
        .map(|bytes| unsafe { std::str::from_utf8_unchecked(bytes) })
        .map(str::to_lowercase)
}
