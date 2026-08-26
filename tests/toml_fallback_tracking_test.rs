#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;

mod common;

#[test]
fn matched_dynamic_command_is_tracked_as_rtk_command_not_parse_failure() {
    let temp = tempfile::tempdir().expect("tempdir");
    let flutter = temp.path().join("flutter");
    let db = temp.path().join("history.db");
    fs::write(
        &flutter,
        "#!/bin/sh\nprintf '00:00 +1: All tests passed!\\n'\n",
    )
    .expect("write fake flutter");
    fs::set_permissions(&flutter, fs::Permissions::from_mode(0o755)).expect("chmod fake flutter");

    let path = std::env::join_paths(std::iter::once(temp.path().to_path_buf()).chain(
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()),
    ))
    .expect("test PATH");

    let output = common::rtk_command()
        .env("RTK_DB_PATH", &db)
        .env("PATH", path)
        .args(["flutter", "test"])
        .output()
        .expect("run rtk flutter test");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let conn = rusqlite::Connection::open(&db).expect("open tracking DB");
    let rtk_cmd: String = conn
        .query_row("SELECT rtk_cmd FROM commands", [], |row| row.get(0))
        .expect("tracked command");
    let failures: i64 = conn
        .query_row("SELECT COUNT(*) FROM parse_failures", [], |row| row.get(0))
        .expect("failure count");

    assert_eq!(rtk_cmd, "rtk flutter test");
    assert_eq!(failures, 0);
}
