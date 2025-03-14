use crate::workdir::Workdir;

#[test]
fn comments() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["# test file to see how comments work", ""],
            svec!["# this is another comment before the header", ""],
            svec!["# DATA DICTIONARY", ""],
            svec!["# column1 - alphabetic; id of the column", ""],
            svec!["# column2 - numeric; just a number", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["#b", "2"],
            svec!["c", "3"],
            svec!["#d - this row is corrupted skip", "extra col2"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.env("QSV_COMMENT_CHAR", "#");
    cmd.arg("comments.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn comments_long() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["# test file to see how comments work", ""],
            svec!["# this is another comment before the header", ""],
            svec!["# DATA DICTIONARY", ""],
            svec!["# column1 - alphabetic; id of the column", ""],
            svec!["# column2 - numeric; just a number", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["#b", "2"],
            svec!["c", "3"],
            svec!["#d - this row is corrupted skip", "extra col2"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.env("QSV_COMMENT_CHAR", "# is the comment character");
    cmd.arg("comments.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn comments_unicode_supported() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["Ǽ test file to see how comments work", ""],
            svec!["Ǽ yet another comment", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["Ǽb", "2"],
            svec!["c", "3"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.env("QSV_COMMENT_CHAR", "Ǽ");
    cmd.arg("comments.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn comments_count() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["Ǽ test file to see how comments work", ""],
            svec!["Ǽ yet another comment", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["Ǽb", "2"],
            svec!["c", "3"],
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.env("QSV_COMMENT_CHAR", "Ǽ");
    cmd.arg("comments.csv");

    let got_count: usize = wrk.stdout(&mut cmd);
    rassert_eq!(got_count, 2);
}

#[test]
fn truncate_comment_char() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["Ǽ test file to see how comments work", ""],
            svec!["Ǽ yet another comment", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["Ǽb", "2"],
            svec!["c", "3"],
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.env("QSV_COMMENT_CHAR", "comment");
    cmd.arg("comments.csv");

    let got_count: usize = wrk.stdout(&mut cmd);
    // we only process the first character passed, so we used 'c' as the comment_char
    rassert_eq!(got_count, 3);
}

#[test]
fn comments_headers() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["// test file to see how comments work", ""],
            svec!["// yet another comment", ""],
            svec!["/ still a comment", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["//b", "2"],
            svec!["c", "3"],
        ],
    );
    let mut cmd = wrk.command("headers");
    cmd.env("QSV_COMMENT_CHAR", "/");
    cmd.arg("comments.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
1   column1
2   column2";
    assert_eq!(got, expected);
}

#[test]
fn envlist() {
    let wrk = Workdir::new("envlist");
    let mut cmd = wrk.command("");
    cmd.env("QSV_ENVVAR", "#");
    cmd.env("MIMALLOC_ENVVAR", "1");
    cmd.arg("--envlist");

    let expected = "MIMALLOC_ENVVAR: \"1\"\nQSV_ENVVAR: \"#\"";
    let got: String = wrk.stdout(&mut cmd);
    // compare only the first few bytes, as there may be other env vars active
    // e.g. debugging, logging, etc.
    let len = std::cmp::min(got.len(), expected.len());
    assert_eq!(&got[0..len], &expected[0..len]);

    // unset it so we don't have side effects outside tests
    // as these env vars persists
    cmd.env("QSV_ENVVAR", "");
    cmd.env("MIMALLOC_ENVVAR", "");
}

#[test]
fn test_input_skiplines() {
    let wrk = Workdir::new("input_skiplines");
    wrk.create(
        "preamble.csv",
        vec![
            svec!["# test file to see how skiplines work", ""],
            svec!["! this is another comment before the header", ""],
            svec!["# DATA DICTIONARY", ""],
            svec!["! column1 - alphabetic; id of the column", ""],
            svec!["% column2 - numeric; just a number", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["c", "3"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lines").arg("5").arg("preamble.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_autoskip() {
    let wrk = Workdir::new("input_autoskip");
    let test_file = wrk.load_test_file("snifftest.csv");

    let mut cmd = wrk.command("input");
    cmd.arg("--auto-skip").arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2", "h3", "h4"],
        svec!["abcdefg", "1", "a", "3.14"],
        svec!["a", "2", "z", "1.2020569"],
        svec!["c", "42", "x", "1.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_skip_one_line() {
    let wrk = Workdir::new("input_skip_one_line");
    wrk.create(
        "preamble.csv",
        vec![
            svec!["# test file to see how skiplines work", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["c", "3"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lines").arg("1").arg("preamble.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_skip_no_line() {
    let wrk = Workdir::new("input_skip_no_line");
    wrk.create(
        "preamble.csv",
        vec![
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["c", "3"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lines").arg("0").arg("preamble.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_trim_headers() {
    let wrk = Workdir::new("input_trim_headers");
    wrk.create(
        "data.csv",
        vec![
            svec!["   column1   ", "  column2   "],
            svec!["  a", "1"],
            svec!["c  ", "3"],
            svec!["e", "5   "],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--trim-headers").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["  a", "1"],
        svec!["c  ", "3"],
        svec!["e", "5   "],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_trim_fields() {
    let wrk = Workdir::new("input_trim_fields");
    wrk.create(
        "data.csv",
        vec![
            svec!["column1   ", "column2   "],
            svec!["   a", "  1"],
            svec!["c   ", "3  "],
            svec!["   e   ", "  5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--trim-fields").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1   ", "column2   "],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_trim_headers_fields() {
    let wrk = Workdir::new("input_trim_headers_fields");
    wrk.create(
        "data.csv",
        vec![
            svec!["   column1   ", "   column2   "],
            svec!["   a", "  1"],
            svec!["c   ", "3  "],
            svec!["   e   ", "  5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--trim-headers")
        .arg("--trim-fields")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_skip_lastlines() {
    let wrk = Workdir::new("input_skip_lastlines");
    wrk.create(
        "data.csv",
        vec![
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
            svec!["d", "4"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lastlines").arg("2").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["b", "2"],
        svec!["c", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_skip_lines_both() {
    let wrk = Workdir::new("input_skip_lines_both");
    wrk.create(
        "data.csv",
        vec![
            svec!["#column1", "column2"],
            svec!["! column1", "column2"],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
            svec!["d", "4"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lastlines")
        .arg("2")
        .arg("--skip-lines")
        .arg("2")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["b", "2"],
        svec!["c", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_both_skip_flexible() {
    let wrk = Workdir::new("test_input_both_skip_flexible");

    let test_file = wrk.load_test_file("inputskiptest.csv");

    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lastlines")
        .arg("4")
        .arg("--skip-lines")
        .arg("5")
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["b", "2"],
        svec!["c", "3"],
    ];
    assert_eq!(got, expected);
}
