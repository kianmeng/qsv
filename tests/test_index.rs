use std::fs;

use filetime::{set_file_times, FileTime};

use crate::workdir::Workdir;

#[test]
fn index_outdated_count() {
    let wrk = Workdir::new("index_outdated_count");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
        ],
    );

    let md = fs::metadata(&wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        &wrk.path("in.csv"),
        future_time(FileTime::from_last_modification_time(&md)),
        future_time(FileTime::from_last_access_time(&md)),
    )
    .unwrap();

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");

    // count works even if index is stale
    let expected_count = 2;
    let got_count: usize = wrk.stdout(&mut cmd);
    rassert_eq!(got_count, expected_count);
}

#[test]
fn index_outdated_stats() {
    let wrk = Workdir::new("index_outdated_stats");

    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
        ],
    );

    let md = fs::metadata(&wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        &wrk.path("in.csv"),
        future_time(FileTime::from_last_modification_time(&md)),
        future_time(FileTime::from_last_access_time(&md)),
    )
    .unwrap();

    // sleep for a bit to ensure file system updates the file metadata properly
    // as this test intermittently fails without the sleep
    std::thread::sleep(std::time::Duration::from_millis(100));

    // stats should fail if the index is stale
    let mut cmd = wrk.command("stats");
    cmd.env_clear().arg("in.csv");

    wrk.assert_err(&mut cmd);
}

fn future_time(ft: FileTime) -> FileTime {
    let secs = ft.unix_seconds();
    FileTime::from_unix_time(secs + 10_000, 0)
}
