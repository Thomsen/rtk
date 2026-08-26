use rusqlite::{params, Connection};

mod common;

fn seed_db(path: &std::path::Path) {
    let conn = Connection::open(path).expect("open test DB");
    conn.execute_batch(
        "CREATE TABLE commands (
            id INTEGER PRIMARY KEY,
            timestamp TEXT NOT NULL,
            original_cmd TEXT NOT NULL,
            rtk_cmd TEXT NOT NULL,
            input_tokens INTEGER NOT NULL,
            output_tokens INTEGER NOT NULL,
            saved_tokens INTEGER NOT NULL,
            savings_pct REAL NOT NULL,
            exec_time_ms INTEGER DEFAULT 0,
            project_path TEXT DEFAULT ''
        );",
    )
    .expect("create commands table");

    for (index, command) in ["rtk alpha", "rtk beta,arg", "rtk gamma"]
        .into_iter()
        .enumerate()
    {
        conn.execute(
            "INSERT INTO commands
             (timestamp, original_cmd, rtk_cmd, input_tokens, output_tokens,
              saved_tokens, savings_pct, exec_time_ms, project_path)
             VALUES (?1, ?2, ?3, ?4, 0, ?4, 100.0, 1, '')",
            params!["2026-01-01T00:00:00Z", command, command, index + 1],
        )
        .expect("insert command");
    }
}

#[test]
fn gain_exports_all_or_limited_command_rows() {
    let temp = tempfile::tempdir().expect("tempdir");
    let db = temp.path().join("history.db");
    seed_db(&db);

    let json_output = common::rtk_command()
        .env("RTK_DB_PATH", &db)
        .args(["gain", "--commands", "all", "--format", "json"])
        .output()
        .expect("gain json");
    assert!(json_output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&json_output.stdout).expect("valid JSON");
    assert_eq!(json["commands"].as_array().map(Vec::len), Some(3));

    let csv_output = common::rtk_command()
        .env("RTK_DB_PATH", &db)
        .args(["gain", "--commands", "2", "--format", "csv"])
        .output()
        .expect("gain csv");
    assert!(csv_output.status.success());
    let csv = String::from_utf8_lossy(&csv_output.stdout);
    assert!(csv.contains("command,count,saved_tokens,avg_savings_pct,avg_time_ms"));
    assert!(csv.contains("\"rtk beta,arg\""));
    assert_eq!(
        csv.lines()
            .filter(|line| line.starts_with("rtk ") || line.starts_with('"'))
            .count(),
        2
    );
}

#[test]
fn gain_rejects_zero_command_limit() {
    let temp = tempfile::tempdir().expect("tempdir");
    let db = temp.path().join("history.db");
    seed_db(&db);

    let output = common::rtk_command()
        .env("RTK_DB_PATH", &db)
        .args(["gain", "--commands", "0"])
        .output()
        .expect("gain invalid limit");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("greater than zero"));
}
