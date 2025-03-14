## qsv: Ultra-fast CSV data-wrangling toolkit

[![Linux build status](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml)
[![Windows build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml)
[![macOS build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml)
[![Security audit](https://github.com/jqnatividad/qsv/actions/workflows/security-audit.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/security-audit.yml)
[![Downloads](https://img.shields.io/github/downloads/jqnatividad/qsv/total?logo=github)](https://github.com/jqnatividad/qsv/releases/latest)
[![Clones](https://img.shields.io/badge/dynamic/json?color=success&label=clones&query=count&url=https://gist.githubusercontent.com/jqnatividad/13f60ad0b54856a55f60b8e653079349/raw/clone.json&logo=github)](https://github.com/MShawon/github-clone-count-badge)
[![Discussions](https://img.shields.io/github/discussions/jqnatividad/qsv)](https://github.com/jqnatividad/qsv/discussions)
[![Docs](https://img.shields.io/badge/wiki-docs-yellowgreen)](https://github.com/jqnatividad/qsv/wiki)
[![Minimum supported Rust version](https://img.shields.io/badge/Rust-1.63-red?logo=rust)](#minimum-supported-rust-version)
[![Crates.io](https://img.shields.io/crates/v/qsv.svg)](https://crates.io/crates/qsv)
[![Crates.io downloads](https://img.shields.io/crates/d/qsv?color=orange&label=crates.io%20downloads)](https://crates.io/crates/qsv)

<div align="center">

 &nbsp;          |  Table of Contents
:-------------------------:|:-------------------------
![qsv logo](docs/images/qsv-logo.png)  |qsv is a command line program for<br>indexing, slicing, analyzing, filtering,<br>enriching, validating & joining CSV files.<br>Commands are simple, fast & composable.<br><br>* [Available Commands](#available-commands)<br>* [Installation](#installation)<br> * [Whirlwind Tour](docs/whirlwind_tour.md#a-whirlwind-tour)<br>* [Cookbook](https://github.com/jqnatividad/qsv/wiki)<br>* [FAQ](https://github.com/jqnatividad/qsv/discussions/categories/faq)<br>* [Changelog](https://github.com/jqnatividad/qsv/blob/master/CHANGELOG.md#changelog)<br>* [Performance Tuning](https://github.com/jqnatividad/qsv/blob/master/docs/PERFORMANCE.md#performance-tuning)<br>* [Benchmarks](docs/BENCHMARKS.md)<br>* [NYC School of Data 2022 slides](https://docs.google.com/presentation/d/e/2PACX-1vQ12ndZL--gkz0HLQRaxqsNOwzddkv1iUKB3sq661yA77OPlAsmHJHpjaqt9s9QEf73VqMfb0cv4jHU/pub?start=false&loop=false&delayms=3000)<br>* [Sponsor](#sponsor)

</div>

> ℹ️ **NOTE:** qsv is a fork of the popular [xsv](https://github.com/BurntSushi/xsv) utility, merging several pending PRs [since xsv 0.13.0's May 2018 release](https://github.com/BurntSushi/xsv/issues/267). It also has numerous new features & 56 additional commands/subcommands/operations (for a total of 76).
See [FAQ](https://github.com/jqnatividad/qsv/discussions/categories/faq) for more details.

## Available commands

| Command | Description |
| --- | --- |
| [apply](/src/cmd/apply.rs#L2)[^1][^5] | Apply series of string, date, currency & geocoding transformations to a CSV column. It also has some basic [NLP](https://en.wikipedia.org/wiki/Natural_language_processing) functions ([similarity](https://crates.io/crates/strsim), [sentiment analysis](https://crates.io/crates/vader_sentiment), [profanity](https://docs.rs/censor/latest/censor/), [eudex](https://github.com/ticki/eudex#eudex-a-blazingly-fast-phonetic-reductionhashing-algorithm) & [language detection](https://crates.io/crates/whatlang)).  |
| [behead](/src/cmd/behead.rs#L2) | Drop headers from a CSV.  |
| [cat](/src/cmd/cat.rs#L2) | Concatenate CSV files by row or by column. |
| [count](/src/cmd/count.rs#L2)[^2] | Count the rows in a CSV file. (Instantaneous with an index.) |
| [dedup](/src/cmd/dedup.rs#L2)[^3][^5] | Remove redundant rows (See also `sortcheck` command). |
| [enum](/src/cmd/enumerate.rs#L2) | Add a new column enumerating rows by adding a column of incremental or uuid identifiers. Can also be used to copy a column or fill a new column with a constant value.  |
| [excel](/src/cmd/excel.rs#L2) | Exports a specified Excel/ODS sheet to a CSV file. |
| [exclude](/src/cmd/exclude.rs#L2)[^2] | Removes a set of CSV data from another set based on the specified columns.  |
| [explode](/src/cmd/explode.rs#L2) | Explode rows into multiple ones by splitting a column value based on the given separator.  |
| [extsort](/src/cmd/extsort.rs#L2)[^5] | Sort an arbitrarily large CSV/text file using a multithreaded [external merge sort](https://en.wikipedia.org/wiki/External_sorting) algorithm. |
| [fetch](/src/cmd/fetch.rs#L2) | Fetches data from web services for every row using **HTTP Get**. Comes with [jql](https://github.com/yamafaktory/jql#%EF%B8%8F-usage) JSON query language support, dynamic throttling ([RateLimit](https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html)) & caching with optional [Redis](https://redis.io/) support for persistent caching. |
| [fetchpost](/src/cmd/fetchpost.rs#L2) | Similar to `fetch`, but uses **HTTP Post**. ([HTTP GET vs POST methods](https://www.geeksforgeeks.org/difference-between-http-get-and-post-methods/)) |
| [fill](/src/cmd/fill.rs#L2) | Fill empty values.  |
| [fixlengths](/src/cmd/fixlengths.rs#L2) | Force a CSV to have same-length records by either padding or truncating them. |
| [flatten](/src/cmd/flatten.rs#L2) | A flattened view of CSV records. Useful for viewing one record at a time.<br />e.g. `qsv slice -i 5 data.csv \| qsv flatten`. |
| [fmt](/src/cmd/fmt.rs#L2) | Reformat a CSV with different delimiters, record terminators or quoting rules. (Supports ASCII delimited data.)  |
| [foreach](/src/cmd/foreach.rs#L3)[^1] | Loop over a CSV to execute bash commands. (not available on Windows)  |
| [frequency](/src/cmd/frequency.rs#L2)[^2][^4] | Build [frequency tables](https://statisticsbyjim.com/basics/frequency-table/) of each column. (Uses multithreading to go faster if an index is present.) |
| [generate](/src/cmd/generate.rs#L2)[^1] | Generate test data by profiling a CSV using [Markov decision process](https://crates.io/crates/test-data-generation) machine learning.  |
| [headers](/src/cmd/headers.rs#L2) | Show the headers of a CSV. Or show the intersection of all headers between many CSV files. |
| [index](/src/cmd/index.rs#L2) | Create an index for a CSV. This is very quick & provides constant time indexing into the CSV file. Also enables multithreading for `frequency`, `split`, `stats` and `schema` commands. |
| [input](/src/cmd/input.rs#L2)[^2] | Read CSV data with special quoting, trimming, line-skipping and UTF-8 transcoding rules. Typically used to "normalize" a CSV for further processing with other qsv commands. |
| [join](/src/cmd/join.rs#L2)[^2] | Inner, outer, cross, anti & semi joins. Uses a simple hash index to make it fast.  |
| [jsonl](/src/cmd/jsonl.rs#L2) | Convert newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)) to CSV. See `tojsonl` command to convert CSV to JSONL.
| [lua](/src/cmd/lua.rs#L2)[^1] | Execute a [Lua](https://www.lua.org/about.html) 5.4.4 script over CSV lines to transform, aggregate or filter them.  |
| [partition](/src/cmd/partition.rs#L2) | Partition a CSV based on a column value. |
| [pseudo](/src/cmd/pseudo.rs#L2) | [Pseudonymise](https://en.wikipedia.org/wiki/Pseudonymization) the value of the given column by replacing them with an incremental identifier.  |
| [py](/src/cmd/python.rs#L2)[^1] | Evaluate a Python expression over CSV lines to transform, aggregate or filter them. Python's [f-strings](https://www.freecodecamp.org/news/python-f-strings-tutorial-how-to-use-f-strings-for-string-formatting/) is particularly useful for extended formatting (Python 3.8+ required).  |
| [rename](/src/cmd/rename.rs#L2) |  Rename the columns of a CSV efficiently.  |
| [replace](/src/cmd/replace.rs#L2) | Replace CSV data using a regex.  |
| [reverse](/src/cmd/reverse.rs#L2)[^3] | Reverse order of rows in a CSV. Unlike the `sort --reverse` command, it preserves the order of rows with the same key.  |
| [sample](/src/cmd/sample.rs#L2)[^2] | Randomly draw rows (with optional seed) from a CSV using [reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling) (i.e., use memory proportional to the size of the sample).  |
| [schema](/src/cmd/schema.rs#L2)[^4] | Infer schema from CSV data and output in [JSON Schema](https://json-schema.org/) format. Uses multithreading to go faster if an index is present. See `validate` command to use the generated JSON Schema to validate if similar CSVs comply with the schema. |
| [search](/src/cmd/search.rs#L2) | Run a regex over a CSV. Applies the regex to each field individually & shows only matching rows.  |
| [searchset](/src/cmd/searchset.rs#L2) | **Run multiple regexes over a CSV in a single pass.** Applies the regexes to each field individually & shows only matching rows.  |
| [select](/src/cmd/select.rs#L2) | Select, re-order, duplicate or drop columns.  |
| [slice](/src/cmd/slice.rs#L2)[^2][^3] | Slice rows from any part of a CSV. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice).  |
| [sniff](/src/cmd/sniff.rs#L2)[^2] | Quickly sniff CSV metadata (delimiter, header row, preamble rows, quote character, flexible, is_utf8, number of records, number of fields, field names & data types). |
| [sort](/src/cmd/sort.rs#L2)[^5] | Sorts CSV data in alphabetical, numerical, reverse or random (with optional seed) order (See also `extsort` & `sortcheck` commands).  |
| [sortcheck](/src/cmd/sortcheck.rs#L2)[^2] | Check if a CSV is sorted. With the --json options, also retrieve record count, sort breaks & duplicate count. |
| [split](/src/cmd/split.rs#L2)[^2][^4] | Split one CSV file into many CSV files of N chunks. (Uses multithreading to go faster if an index is present.) |
| [stats](/src/cmd/stats.rs#L2)[^2][^3][^4] | Infer data type (Null, String, Float, Integer, Date, DateTime) & compute descriptive statistics for each column in a CSV (sum, min/max, min/max length, mean, stddev, variance, nullcount, quartiles, IQR, lower/upper fences, skewness, median, mode & cardinality). Uses multithreading to go faster if an index is present. |
| [table](/src/cmd/table.rs#L2)[^3] | Show aligned output of a CSV using [elastic tabstops](https://github.com/BurntSushi/tabwriter).  |
| [tojsonl](/src/cmd/tojsonl.rs#L2)[^4] | Converts CSV to a newline-delimited JSON (JSONL/NDJSON). See `jsonl` command to convert JSONL to CSV. |
| [transpose](/src/cmd/transpose.rs#L2)[^3] | Transpose rows/columns of a CSV.  |
| [validate](/src/cmd/validate.rs#L2)[^2][^5] | Validate CSV data with a JSON Schema (See `schema` command). If no jsonschema file is provided, validates if a CSV conforms to the [RFC 4180 standard](https://datatracker.ietf.org/doc/html/rfc4180). |

[^1]: enabled by optional feature flag. Not available on `qsvlite` & `qsvdp`.
[^2]: uses an index when available.   
[^3]: loads the entire CSV into memory. Note that `dedup`, `stats` & `transpose` have modes that do not load the entire CSV into memory.   
[^4]: multithreaded when an index is available.   
[^5]: multithreaded even without an index.

## Installation

Pre-built binaries for Windows, Linux and macOS are available [for download](https://github.com/jqnatividad/qsv/releases/latest), including binaries compiled with [Rust Nightly/Unstable](https://stackoverflow.com/questions/70745970/rust-nightly-vs-beta-version) ([more info](https://github.com/jqnatividad/qsv/blob/master/docs/PERFORMANCE.md#nightly-release-builds)).

There are four variants of qsv:
 * `qsv` enables all [features](#feature-flags) valid for the target platform[^6]
 * `qsvnp` enables all features EXCEPT python ("np" stands for "no python") 
 * `qsvlite` has all features disabled (~half the size of `qsv`)
 * `qsvdp` is optimized for use with [DataPusher+](https://github.com/dathere/datapusher-plus), with only DataPusher+ relevant commands and the self-update engine removed (~sixth of the size of `qsv`).

Alternatively, you can install from source by [installing Cargo](https://crates.io/install)
([Rust's](https://www.rust-lang.org/) package manager) and installing `qsv` using Cargo:

```bash
cargo install qsv --features all_full
```

If you encounter compilation errors, ensure you have at least Python 3.8 installed and you're using the exact
version of the dependencies qsv was built with by issuing:

```bash
cargo install qsv --locked --features all_full
```

The binary will be installed in `~/.cargo/bin`.

Compiling from source also works similarly:

```bash
git clone git@github.com:jqnatividad/qsv.git
cd qsv
cargo build --release --features all_full
# or if you encounter compilation errors
cargo build --release --locked --features all_full
```

The compiled binary will end up in `./target/release/`.

To enable optional features, use cargo `--features` (see [Feature Flags](#feature-flags) for more info):

```bash
cargo install qsv --features apply,generate,lua,fetch,foreach,python,self_update,full
# or shorthand
cargo install qsv --features all_full
# or to install all features EXCEPT python
cargo install qsv --features nopython_full
# or to install qsvlite
cargo install qsv --features lite
# or to install qsvdp
cargo install qsv --features datapusher_plus

# or when compiling from a local repo
cargo build --release --features apply,generate,lua,fetch,foreach,python,self_update,full
# shorthand
cargo build --release --features all_full
# all features EXCEPT python
cargo build --release --features nopython_full
# for qsvlite
cargo build --release --features lite
# for qsvdp
cargo build --release --features datapusher_plus
```

[^6]: The `foreach` feature is not available on Windows. The `python` feature is not enabled on cross-compiled pre-built binaries as we don't have
access to a native python interpreter for those platforms (aarch64, i686, and arm) on GitHub's x86_64-based action runners. Compile natively on those platforms with Python 3.8+ installed, if you want to enable the `python` feature.

### Minimum Supported Rust Version

qsv's MSRV policy is to require [Rust stable](https://github.com/rust-lang/rust/blob/master/RELEASES.md) - currently version 1.63.

## Tab Completion

qsv's command-line options are quite extensive. Thankfully, since it uses [docopt](http://docopt.org/) for CLI processing,
we can take advantage of [docopt.rs' tab completion support](https://github.com/docopt/docopt.rs#tab-completion-support) to make it
easier to use qsv at the command-line (currently, only bash shell is supported):

```bash
# install docopt-wordlist
cargo install docopt

# IMPORTANT: run these commands from the root directory of your qsv git repository
# to setup bash qsv tab completion
echo "DOCOPT_WORDLIST_BIN=\"$(which docopt-wordlist)"\" >> $HOME/.bash_completion
echo "source \"$(pwd)/scripts/docopt-wordlist.bash\"" >> $HOME/.bash_completion
echo "complete -F _docopt_wordlist_commands qsv" >> $HOME/.bash_completion
```

## File formats

qsv recognizes UTF-8/ASCII encoded, CSV (`.csv`) and TSV files (`.tsv` and `.tab`). CSV files are assummed to have "," (comma) as a delimiter,
and TSV files, "\t" (tab) as a delimiter. The delimiter is a single ascii character that can be set either by the `--delimiter` command-line option or
with the `QSV_DEFAULT_DELIMITER` environment variable or automatically detected when `QSV_SNIFF_DELIMITER` is set.

When using the `--output` option, note that qsv will UTF-8 encode the file and automatically change the delimiter used in the generated file based on the file extension - i.e. comma for `.csv`, tab for `.tsv` and `.tab` files.

[JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/) files are also recognized and converted from/to CSV with the [`jsonl`](/src/cmd/jsonl.rs#L11) and [`tojsonl`](/src/cmd/tojsonl.rs#L12) commands respectively.

The `fetch` & `fetchpost` commands also produces JSONL files when its invoked without the `--new-column` option, and TSV files with the `--report` option.

The `sniff`, `sortcheck` and `validate` commands produce JSON files with their `--json` and `--pretty-json` options.

The `schema` command produces a [JSON Schema Validation (Draft 7)](https://json-schema.org/draft/2020-12/json-schema-validation.html) file with the ".schema.json" file extension, which can be used with the `validate` command.

The `excel` command recognizes Excel and Open Document Spreadsheet(ODS) files (`.xls`, `.xlsx`, `.xlsm`, `.xlsb` and `.ods` files).

### RFC 4180

qsv validates against the [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180) CSV standard. However IRL, CSV formats vary significantly and qsv is actually not strictly compliant with the specification so it can process "real-world" CSV files.
qsv leverages the awesome [Rust CSV](https://docs.rs/csv/latest/csv/) library, which in turn, is built on top of the [csv-core](https://docs.rs/csv-core/latest/csv_core/index.html) library to read/write CSV files.

Click [here](https://docs.rs/csv-core/latest/csv_core/struct.Reader.html#rfc-4180) to find out more about how qsv conforms to the standard with `csv-core`.

### **UTF-8 Encoding**

The following commands require UTF-8 encoded input (of which ASCII is a subset) - `dedup`, `exclude`, `fetch`, `fetchpost`, `frequency`, `join`, `schema`, `sort`, `stats` & `validate`.

For these commands, qsv checks if the input is UTF-8 encoded by scanning the first 8k, and will abort if its not unless `QSV_SKIPUTF8_CHECK` is set. On Linux and macOS, UTF-8 encoding is the default.

This was done to increase performance of these commands, as they make extensive use of `from_utf8_unchecked` so as not to pay the repetitive utf-8 validation penalty, no matter how small, even for already utf-8 encoded files.

Should you need to re-encode CSV/TSV files, you can use the `input` command to transcode to UTF-8. It will replace all invalid UTF-8 sequences with `�`. Alternatively, there are several utilities you can use to do so on [Linux/macOS](https://stackoverflow.com/questions/805418/how-can-i-find-encoding-of-a-file-via-a-script-on-linux) and [Windows](https://superuser.com/questions/1163753/converting-text-file-to-utf-8-on-windows-command-prompt).

### **Windows Usage Note**

Unlike other modern operating systems, Microsoft Windows' [default encoding is UTF16-LE](https://stackoverflow.com/questions/66072117/why-does-windows-use-utf-16le). This will cause problems when redirecting qsv's output to a CSV file and trying to open it with Excel (which ignores the comma delimiter, with everything in the first column):

```
qsv stats wcp.csv > wcpstats.csv
```

Which is weird, since you would think [Microsoft's own Excel would properly recognize UTF16-LE encoded CSV files](https://answers.microsoft.com/en-us/msoffice/forum/all/opening-csv-file-with-utf16-encoding-in-excel-2010/ed522cb9-e88d-4b82-b88e-a2d4bd99f874?auth=1). Regardless, to create a properly UTF-8 encoded file, use the `--output` option instead:

```
qsv stats wcp.csv --output wcpstats.csv
```

## Environment Variables

| Variable | Description |
| --- | --- |
| `QSV_DEFAULT_DELIMITER` | single ascii character to use as delimiter.  Overrides `--delimeter` option. Defaults to "," (comma) for CSV files and "\t" (tab) for TSV files when not set. Note that this will also set the delimiter for qsv's output to stdout.<br>However, using the `--output` option, regardless of this environment variable, will automatically change the delimiter used in the generated file based on the file extension - i.e. comma for `.csv`, tab for `.tsv` and `.tab` files. |
| `QSV_SNIFF_DELIMITER` | if set, the delimiter is automatically detected. Overrides `QSV_DEFAULT_DELIMITER` and `--delimiter` option. Note that this does not work with stdin. |
| `QSV_NO_HEADERS` | if set, the first row will **NOT** be interpreted as headers. Supersedes `QSV_TOGGLE_HEADERS`. |
| `QSV_TOGGLE_HEADERS` | if set to `1`, toggles header setting - i.e. inverts qsv header behavior, with no headers being the default, and setting `--no-headers` will actually mean headers will not be ignored. |
| `QSV_AUTOINDEX` | if set, automatically create an index when none is detected. Also automatically update stale indices. |
| `QSV_COMMENT_CHAR` | set to an ascii character. If set, any lines(including the header) that start with this character are ignored. |
| `QSV_MAX_JOBS` | number of jobs to use for multithreaded commands (currently `apply`, `dedup`, `extsort`, `frequency`, `schema`, `sort`, `split`, `stats`, `tojsonl` and `validate`). If not set, max_jobs is set to the detected number of logical processors.  See [Multithreading](docs/PERFORMANCE.md#multithreading) for more info. |
| `QSV_NO_UPDATE` | if set, prohibit self-update version check for the latest qsv release published on GitHub. |
| `QSV_PREFER_DMY` | if set, date parsing will use DMY format. Otherwise, use MDY format (used with `apply datefmt`, `schema`, `sniff` & `stats` commands). |
| `QSV_REGEX_UNICODE` | if set, makes `search`, `searchset` and `replace` commands unicode-aware. For increased performance, these commands are not unicode-aware by default and will ignore unicode values when matching and will abort when unicode characters are used in the regex. Note that the `apply operations regex_replace` operation is always unicode-aware. |
| `QSV_SKIPUTF8_CHECK` | if set, skip UTF-8 encoding check. Otherwise, for several commands that require UTF-8 encoded input (see [UTF8-Encoding](#utf-8-encoding)), qsv scans the first 8k. |
| `QSV_RDR_BUFFER_CAPACITY` | reader buffer size (default (bytes): 16384) |
| `QSV_WTR_BUFFER_CAPACITY` | writer buffer size (default (bytes): 65536) |
| `QSV_LOG_LEVEL` | desired level (default - off; `error`, `warn`, `info`, `trace`, `debug`). |
| `QSV_LOG_DIR` | when logging is enabled, the directory where the log files will be stored. If the specified directory does not exist, qsv will attempt to create it. If not set, the log files are created in the directory where qsv was started. See [Logging](docs/Logging.md#logging) for more info. |
| `QSV_PROGRESSBAR` | if set, enable the --progressbar option on the `apply`, `fetch`, `fetchpost`, `foreach`, `lua`, `py`, `replace`, `search`, `searchset`, `sortcheck` and `validate` commands.  |
| `QSV_REDIS_CONNSTR` | the `fetch` command can use [Redis](https://redis.io/) to cache responses. Set to connect to the desired Redis instance. (default: `redis:127.0.0.1:6379/1`). For more info on valid Redis connection string formats, see https://docs.rs/redis/latest/redis/#connection-parameters. |
| `QSV_FP_REDIS_CONNSTR` | the `fetchpost` command can also use Redis to cache responses (default: `redis:127.0.0.1:6379/2`). Note that `fetchpost` connects to database 2, as opposed to `fetch` which connects to database 1. |
| `QSV_REDIS_MAX_POOL_SIZE` | the maximum Redis connection pool size. (default: 20). |
| `QSV_REDIS_TTL_SECONDS` | set time-to-live of Redis cached values (default (seconds): 2419200 (28 days)). |
| `QSV_REDIS_TTL_REFRESH`| if set, enables cache hits to refresh TTL of cached values. |

Several dependencies also have environment variables that influence qsv's performance & behavior:

* Memory Management ([mimalloc](https://docs.rs/mimalloc/latest/mimalloc/))   
  When incorporating qsv into a data pipeline that runs in batch mode, particularly with very large CSV files using qsv commands that load entire CSV files into memory, you can
  [fine-tune Mimalloc's behavior using its environment variables](https://github.com/microsoft/mimalloc#environment-options).
* Network Access ([reqwest](https://docs.rs/reqwest/latest/reqwest/))   
  qsv uses reqwest for its `fetch`, `validate` and `--update` functions and will honor [proxy settings](https://docs.rs/reqwest/latest/reqwest/index.html#proxies) set through the `HTTP_PROXY`, `HTTPS_PROXY` and `NO_PROXY` environment variables.
  
> ℹ️ **NOTE:** To get a list of all active qsv-relevant environment variables, run `qsv --envlist`.
Relevant env vars are defined as anything that starts with `QSV_` and `MIMALLOC_`, and the proxy variables listed above.

## Feature Flags

`qsv` has several features:

* `mimalloc` (default) - use the mimalloc allocator (see [Memory Allocator](docs/PERFORMANCE.md#memory-allocator) for more info).
* `apply` - enable `apply` command. This swiss-army knife of CSV transformations is very powerful, but it has a lot of dependencies that increases both compile time and binary size.
* `fetch` - enables the `fetch` and `fetchpost` commands.
* `generate` - enable `generate` command.
* `full` - enable to build qsv binary variant which is feature-capable.
* `all_full` - enable to build qsv binary variant with all features enabled (apply,fetch,foreach,generate,lua,python).
* `nopython_full` - enable to build qsvnp binary variant with all features (apply,fetch,foreach,generate,lua) EXCEPT python.
* `lite` - enable to build qsvlite binary variant with all features disabled.
* `datapusher_plus` - enable to build qsvdp binary variant - the [DataPusher+](https://github.com/dathere/datapusher-plus) optimized qsv binary.
* `nightly` - enable to turn on nightly/unstable features in the `rand`, `regex`, `hashbrown`, `parking_lot` and `pyo3` crates when building with Rust nightly/unstable.
* `self_update` - enable self-update engine, checking GitHub for the latest release.

The following "power-user" features can be abused and present "foot-shooting" scenarios:

* `lua` - enable `lua` command. Embeds a [Lua 5.4.4](https://www.lua.org/manual/5.4/manual.html) interpreter into qsv.
* `foreach` - enable `foreach` command (not valid for Windows).
* `python` - enable `py` command (requires Python 3.8+ development libraris). Note that qsv will automatically use the currently activated python version when running in a virtual environment unless there's a python library (libpython.* on Linux/macOS, python*.dll on Windows) in the directory where qsv is located.

> ℹ️ **NOTE:** `qsvlite`, as the name implies, always has **non-default features disabled**. `qsv` can be built with any combination of the above features  using the cargo `--features` & `--no-default-features` flags. The pre-built `qsv` binaries has **all applicable features valid for the target platform**[^6].

## License

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org).

## Sponsor

<div align="center">

|qsv was made possible by|
:-------------------------:|
|[![datHere Logo](docs/images/datHere-logo-withtagline.png)](https://datHere.com)<br>|
|Standards-based, best-of-breed, open source solutions<br>to make your **Data Useful, Usable & Used.**   |

</div>

## Naming Collision

This project is unrelated to [Intel's Quick Sync Video](https://www.intel.com/content/www/us/en/architecture-and-technology/quick-sync-video/quick-sync-video-general.html).
