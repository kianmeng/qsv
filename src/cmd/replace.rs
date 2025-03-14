static USAGE: &str = "
Replace occurrences of a pattern across a CSV file.

You can of course match groups using parentheses and use those in
the replacement string. But don't forget to escape your $ in bash by using a
backslash or by wrapping the replacement string into single quotes:

  $ qsv replace 'hel(lo)' 'hal$1' file.csv
  $ qsv replace \"hel(lo)\" \"hal\\$1\" file.csv

Returns exitcode 0 when replacements are done, returning number of replacements to stderr.
Returns exitcode 1 when no replacements are done.

Usage:
    qsv replace [options] <pattern> <replacement> [<input>]
    qsv replace --help

replace arguments:
    <pattern>              Regular expression to match.
    <replacement>          Replacement string. Set to '<NULL>' if you want to
                           replace matches with ''.
replace options:
    -i, --ignore-case      Case insensitive search. This is equivalent to
                           prefixing the regex with '(?i)'.
    -s, --select <arg>     Select the columns to search. See 'qsv select -h'
                           for the full syntax.
    -u, --unicode          Enable unicode support. When enabled, character classes
                           will match all unicode word characters instead of only
                           ASCII word characters. Decreases performance.
    --size-limit <mb>      Set the approximate size limit (MB) of the compiled
                           regular expression. If the compiled expression exceeds this 
                           number, then a compilation error is returned.
                           [default: 50]
    --dfa-size-limit <mb>  Set the approximate size of the cache (MB) used by the regular
                           expression engine's Discrete Finite Automata.
                           [default: 10]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -p, --progressbar      Show progress bars. Not valid for stdin.

";

use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::util;
use crate::CliError;
use crate::CliResult;
#[cfg(any(feature = "full", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
use regex::bytes::RegexBuilder;
use serde::Deserialize;
use std::borrow::Cow;
use std::env;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    arg_pattern: String,
    arg_replacement: String,
    flag_select: SelectColumns,
    flag_unicode: bool,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_ignore_case: bool,
    flag_size_limit: usize,
    flag_dfa_size_limit: usize,
    flag_progressbar: bool,
}

const NULL_VALUE: &str = "<NULL>";

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let regex_unicode = match env::var("QSV_REGEX_UNICODE") {
        Ok(_) => true,
        Err(_) => args.flag_unicode,
    };
    let pattern = RegexBuilder::new(&args.arg_pattern)
        .case_insensitive(args.flag_ignore_case)
        .unicode(regex_unicode)
        .size_limit(args.flag_size_limit * (1 << 20))
        .dfa_size_limit(args.flag_dfa_size_limit * (1 << 20))
        .build()?;
    let replacement = if args.arg_replacement == NULL_VALUE {
        b""
    } else {
        args.arg_replacement.as_bytes()
    };
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .checkutf8(false)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    // NOTE: using vec lookups is not the fastest thing in the world but
    // I am not sure it would be worthwhile to rely on a set structure
    let sel_indices = sel.to_vec();

    if !rconfig.no_headers {
        wtr.write_record(&headers)?;
    }

    // prep progress bar
    #[cfg(any(feature = "full", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || std::env::var("QSV_PROGRESSBAR").is_ok()) && !rconfig.is_stdin();
    #[cfg(any(feature = "full", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    #[cfg(any(feature = "full", feature = "lite"))]
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let mut record = csv::ByteRecord::new();
    let mut total_match_ctr: u64 = 0;
    #[cfg(any(feature = "full", feature = "lite"))]
    let mut rows_with_matches_ctr: u64 = 0;
    #[cfg(any(feature = "full", feature = "lite"))]
    let mut match_found;

    while rdr.read_byte_record(&mut record)? {
        #[cfg(any(feature = "full", feature = "lite"))]
        if show_progress {
            progress.inc(1);
        }

        #[cfg(any(feature = "full", feature = "lite"))]
        {
            match_found = false;
        }
        record = record
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                if sel_indices.contains(&i) {
                    if pattern.is_match(v) {
                        total_match_ctr += 1;
                        #[cfg(any(feature = "full", feature = "lite"))]
                        {
                            match_found = true;
                        }
                        pattern.replace_all(v, replacement)
                    } else {
                        Cow::Borrowed(v)
                    }
                } else {
                    Cow::Borrowed(v)
                }
            })
            .collect();

        #[cfg(any(feature = "full", feature = "lite"))]
        if match_found {
            rows_with_matches_ctr += 1;
        }

        wtr.write_byte_record(&record)?;
    }

    wtr.flush()?;

    #[cfg(any(feature = "full", feature = "lite"))]
    if show_progress {
        progress.set_message(format!(
            r#" - {} total matches replaced with "{}" in {} out of {} records."#,
            HumanCount(total_match_ctr),
            args.arg_replacement,
            HumanCount(rows_with_matches_ctr),
            HumanCount(progress.length().unwrap()),
        ));
        util::finish_progress(&progress);
    }

    eprintln!("{total_match_ctr}");
    if total_match_ctr == 0 {
        return Err(CliError::NoMatch());
    }

    Ok(())
}
