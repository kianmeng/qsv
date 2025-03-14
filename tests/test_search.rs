use crate::workdir::Workdir;

fn data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["barfoo", "foobar"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    if headers {
        rows.insert(0, svec!["h1", "h2"]);
    }
    rows
}

#[test]
fn search() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_match() {
    let wrk = Workdir::new("search_match");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");
}

#[test]
fn search_match_quick() {
    let wrk = Workdir::new("search_match_quick");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^a").arg("--quick").arg("data.csv");

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");
    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "");
}

#[test]
fn search_nomatch() {
    let wrk = Workdir::new("search_nomatch");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("waldo").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn search_empty() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("xxx").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn search_empty_no_headers() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("xxx").arg("data.csv");
    cmd.arg("--no-headers");

    wrk.assert_err(&mut cmd);
}

#[test]
fn search_ignore_case() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^FoO").arg("data.csv");
    cmd.arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_unicode() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^Ḟoo").arg("data.csv");
    cmd.arg("--unicode");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_unicode_envvar() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.env("QSV_REGEX_UNICODE", "1");
    cmd.arg("^Ḟoo").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_no_headers() {
    let wrk = Workdir::new("search_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["foobar", "barfoo"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_select() {
    let wrk = Workdir::new("search_select");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--select").arg("h2");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_select_no_headers() {
    let wrk = Workdir::new("search_select_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--select").arg("2");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_invert_match() {
    let wrk = Workdir::new("search_invert_match");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    assert_eq!(got, expected);

    let got = wrk.output_stderr(&mut cmd);
    let expected = "2\n";
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_invert_match_no_headers() {
    let wrk = Workdir::new("search_invert_match");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--invert-match");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["a", "b"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag() {
    let wrk = Workdir::new("search_flag");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").args(["--flag", "flagged"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo", "flagged"],
        svec!["a", "b", "0"],
        svec!["barfoo", "foobar", "3"],
        svec!["Ḟooƀar", "ḃarḟoo", "0"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag_invert_match() {
    let wrk = Workdir::new("search_flag");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").args(["--flag", "flagged"]);
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo", "flagged"],
        svec!["a", "b", "2"],
        svec!["barfoo", "foobar", "0"],
        svec!["Ḟooƀar", "ḃarḟoo", "4"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}
