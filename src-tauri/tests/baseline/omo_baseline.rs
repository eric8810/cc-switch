use cc_switch_lib::read_omo_local_file;

use super::support::{ensure_test_home, reset_baseline_fs, test_mutex};

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn omo_baseline_local_file_read_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let home = ensure_test_home().to_path_buf();

    let path = home
        .join(".config")
        .join("opencode")
        .join("oh-my-opencode.jsonc");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create omo dir");
    }
    std::fs::write(
        &path,
        r#"{
  "agents": {
    "defaults": {
      "model": "gpt-5"
    }
  },
  "categories": {
    "coding": ["default"]
  }
}"#,
    )
    .expect("write omo config");

    let data = read_omo_local_file().await.expect("read omo local file");
    assert_eq!(data.file_path, path.to_string_lossy());
    assert!(data.agents.is_some());
    assert!(data.categories.is_some());
}
