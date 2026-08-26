use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

fn test_db_path() -> &'static PathBuf {
    static TEST_DB_PATH: OnceLock<PathBuf> = OnceLock::new();
    TEST_DB_PATH.get_or_init(|| {
        std::env::temp_dir().join(format!("rtk-integration-test-{}.db", std::process::id()))
    })
}

pub fn rtk_command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_rtk"));
    command.env("RTK_DB_PATH", test_db_path());
    command
}
