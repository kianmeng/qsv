use std::cmp;

use crate::workdir::Workdir;

use crate::{qcheck, Csv, CsvData};

fn prop_sort(name: &str, rows: CsvData, headers: bool) -> bool {
    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows.clone());

    let mut cmd = wrk.command("sort");
    cmd.arg("in.csv");
    if !headers {
        cmd.arg("--no-headers");
    }

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let mut expected = rows.to_vecs();
    let headers = if headers && !expected.is_empty() {
        expected.remove(0)
    } else {
        vec![]
    };
    expected.sort_by(|r1, r2| iter_cmp(r1.iter(), r2.iter()));
    if !headers.is_empty() {
        expected.insert(0, headers);
    }
    rassert_eq!(got, expected)
}

#[test]
fn prop_sort_headers() {
    fn p(rows: CsvData) -> bool {
        prop_sort("prop_sort_headers", rows, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_sort_no_headers() {
    fn p(rows: CsvData) -> bool {
        prop_sort("prop_sort_no_headers", rows, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn sort_select() {
    let wrk = Workdir::new("sort_select");
    wrk.create("in.csv", vec![svec!["1", "b"], svec!["2", "a"]]);

    let mut cmd = wrk.command("sort");
    cmd.arg("--no-headers")
        .args(["--select", "2"])
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["2", "a"], svec!["1", "b"]];
    assert_eq!(got, expected);
}

#[test]
fn sortcheck_select_notsorted() {
    let wrk = Workdir::new("sortcheck_select_notsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "2"])
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_select_sorted() {
    let wrk = Workdir::new("sortcheck_select_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "1"])
        .arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_select_unsorted() {
    let wrk = Workdir::new("sortcheck_select_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "2"])
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_sorted() {
    let wrk = Workdir::new("sortcheck_simple_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers").arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_simple_unsorted() {
    let wrk = Workdir::new("sortcheck_simple_unsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_all() {
    let wrk = Workdir::new("sortcheck_simple_all");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all").arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_all_json() {
    let wrk = Workdir::new("sortcheck_simple_all_json");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all").arg("--json").arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":2}
"#
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_json() {
    let wrk = Workdir::new("sortcheck_simple_json");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--json").arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":2}
"#
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_all_json_progressbar() {
    let wrk = Workdir::new("sortcheck_simple_all_json_progessbar");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all")
        .arg("--json")
        .arg("--progressbar")
        .arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":2}
"#
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn sort_numeric() {
    let wrk = Workdir::new("sort_numeric");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["LETTER", "b"],
            svec!["2", "c"],
            svec!["1", "d"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        //Non-numerics should be put first
        svec!["LETTER", "b"],
        svec!["1", "d"],
        svec!["2", "c"],
        svec!["10", "a"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_numeric_non_natural() {
    let wrk = Workdir::new("sort_numeric_non_natural");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["8.33", "a"],
            svec!["5", "b"],
            svec!["LETTER", "c"],
            svec!["7.4", "d"],
            svec!["3.33", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        //Non-numerics should be put first
        svec!["LETTER", "c"],
        svec!["3.33", "e"],
        svec!["5", "b"],
        svec!["7.4", "d"],
        svec!["8.33", "a"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_reverse() {
    let wrk = Workdir::new("sort_reverse");
    wrk.create(
        "in.csv",
        vec![svec!["R", "S"], svec!["1", "b"], svec!["2", "a"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-R").arg("--no-headers").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["R", "S"], svec!["2", "a"], svec!["1", "b"]];
    assert_eq!(got, expected);
}

#[test]
fn sort_uniq() {
    let wrk = Workdir::new("sort_unique");
    wrk.create(
        "in.csv",
        vec![
            svec!["number", "letter"],
            svec!["2", "c"],
            svec!["1", "a"],
            svec!["3", "f"],
            svec!["2", "b"],
            svec!["1", "d"],
            svec!["2", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-u").args(["-s", "number"]).arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number", "letter"],
        svec!["1", "a"],
        svec!["2", "c"],
        svec!["3", "f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_random() {
    let wrk = Workdir::new("sort_random");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--random").args(["--seed", "42"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["1", "b"],
        svec!["5", "f"],
        svec!["6", "e"],
        svec!["3", "d"],
        svec!["2", "a"],
        svec!["4", "c"],
    ];
    assert_eq!(got, expected);
}

/// Order `a` and `b` lexicographically using `Ord`
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
