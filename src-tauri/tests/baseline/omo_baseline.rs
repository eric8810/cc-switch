use serde_json::json;

use cc_switch_lib::{
    disable_current_omo_test_hook, get_current_omo_provider_id_test_hook, read_omo_local_file,
    Provider,
};

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

#[test]
fn omo_baseline_current_provider_disable_clears_state_and_file() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let home = ensure_test_home().to_path_buf();
    let state = super::support::create_empty_state();

    let provider = Provider {
        id: "omo-provider-1".to_string(),
        name: "OMO Profile".to_string(),
        settings_config: json!({
            "agents": {
                "defaults": {
                    "model": "gpt-5"
                }
            },
            "categories": {
                "coding": ["default"]
            }
        }),
        website_url: None,
        category: Some("omo".to_string()),
        created_at: Some(1_710_000_000_000),
        sort_index: Some(1),
        notes: None,
        meta: None,
        icon: None,
        icon_color: None,
        in_failover_queue: false,
    };
    state
        .db
        .save_provider("opencode", &provider)
        .expect("save omo provider");
    state
        .db
        .set_omo_provider_current("opencode", &provider.id, "omo")
        .expect("set omo current provider");

    let config_path = home
        .join(".config")
        .join("opencode")
        .join("oh-my-opencode.jsonc");
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).expect("create omo dir");
    }
    std::fs::write(
        &config_path,
        r#"{
  "agents": {
    "defaults": {
      "model": "gpt-5"
    }
  }
}"#,
    )
    .expect("write omo config");

    let current_before = get_current_omo_provider_id_test_hook(&state, "omo")
        .expect("get current omo provider");
    assert_eq!(current_before, "omo-provider-1");
    assert!(config_path.exists());

    disable_current_omo_test_hook(&state, "omo").expect("disable current omo");

    let current_after = get_current_omo_provider_id_test_hook(&state, "omo")
        .expect("get current omo provider after disable");
    assert!(current_after.is_empty());
    assert!(!config_path.exists());
}

#[test]
fn omo_slim_baseline_current_provider_disable_clears_state_and_file() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let home = ensure_test_home().to_path_buf();
    let state = super::support::create_empty_state();

    let provider = Provider {
        id: "omo-slim-provider-1".to_string(),
        name: "OMO Slim Profile".to_string(),
        settings_config: json!({
            "agents": {
                "defaults": {
                    "model": "gpt-5-mini"
                }
            }
        }),
        website_url: None,
        category: Some("omo-slim".to_string()),
        created_at: Some(1_710_000_000_000),
        sort_index: Some(1),
        notes: None,
        meta: None,
        icon: None,
        icon_color: None,
        in_failover_queue: false,
    };
    state
        .db
        .save_provider("opencode", &provider)
        .expect("save omo slim provider");
    state
        .db
        .set_omo_provider_current("opencode", &provider.id, "omo-slim")
        .expect("set omo slim current provider");

    let config_path = home
        .join(".config")
        .join("opencode")
        .join("oh-my-opencode-slim.jsonc");
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).expect("create omo slim dir");
    }
    std::fs::write(
        &config_path,
        r#"{
  "agents": {
    "defaults": {
      "model": "gpt-5-mini"
    }
  }
}"#,
    )
    .expect("write omo slim config");

    let current_before = get_current_omo_provider_id_test_hook(&state, "omo-slim")
        .expect("get current omo slim provider");
    assert_eq!(current_before, "omo-slim-provider-1");
    assert!(config_path.exists());

    disable_current_omo_test_hook(&state, "omo-slim").expect("disable current omo slim");

    let current_after = get_current_omo_provider_id_test_hook(&state, "omo-slim")
        .expect("get current omo slim provider after disable");
    assert!(current_after.is_empty());
    assert!(!config_path.exists());
}
