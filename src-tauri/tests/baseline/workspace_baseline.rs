use cc_switch_lib::{list_daily_memory_files, read_daily_memory_file, write_daily_memory_file};

use super::support::{ensure_test_home, reset_baseline_fs, test_mutex};

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn workspace_baseline_daily_memory_round_trip_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let _home = ensure_test_home();

    write_daily_memory_file("2026-03-09.md".to_string(), "hello memory".to_string())
        .await
        .expect("write daily memory");
    let files = list_daily_memory_files()
        .await
        .expect("list daily memory files");
    let content = read_daily_memory_file("2026-03-09.md".to_string())
        .await
        .expect("read daily memory");

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].filename, "2026-03-09.md");
    assert_eq!(content.as_deref(), Some("hello memory"));
}
