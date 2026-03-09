use std::net::TcpListener;

use serde_json::json;

use cc_switch_lib::{get_claude_settings_path, read_json_file, Provider};

use super::support::{create_empty_state, reset_baseline_fs, test_mutex};

fn reserve_local_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind local port");
    let port = listener
        .local_addr()
        .expect("read local addr")
        .port();
    drop(listener);
    port
}

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn proxy_runtime_baseline_start_takeover_and_restore_claude_live() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let state = create_empty_state();

    let mut proxy_config = state.db.get_proxy_config().await.expect("get proxy config");
    proxy_config.listen_port = reserve_local_port();
    state
        .db
        .update_proxy_config(proxy_config)
        .await
        .expect("update proxy config");

    let provider = Provider::with_id(
        "claude-primary".to_string(),
        "Claude Primary".to_string(),
        json!({
            "env": {
                "ANTHROPIC_AUTH_TOKEN": "provider-token"
            }
        }),
        None,
    );
    state
        .db
        .save_provider("claude", &provider)
        .expect("save claude provider");
    state
        .db
        .set_current_provider("claude", "claude-primary")
        .expect("set current provider");

    let live_before = json!({
        "env": {
            "ANTHROPIC_AUTH_TOKEN": "legacy-live-token",
            "ANTHROPIC_BASE_URL": "https://api.anthropic.com"
        },
        "workspace": {
            "path": "/tmp/demo-workspace"
        }
    });
    let live_path = get_claude_settings_path();
    if let Some(parent) = live_path.parent() {
        std::fs::create_dir_all(parent).expect("create claude dir");
    }
    std::fs::write(
        &live_path,
        serde_json::to_string_pretty(&live_before).expect("serialize live before"),
    )
    .expect("write claude live");

    state
        .proxy_service
        .start()
        .await
        .expect("start proxy server");
    let status_after_start = state
        .proxy_service
        .get_status()
        .await
        .expect("get proxy status after start");
    assert!(status_after_start.running);

    let live_after_start: serde_json::Value =
        read_json_file(&live_path).expect("read live after start");
    assert_eq!(live_after_start, live_before);

    state
        .proxy_service
        .set_takeover_for_app("claude", true)
        .await
        .expect("enable claude takeover");

    let takeover_status = state
        .proxy_service
        .get_takeover_status()
        .await
        .expect("get takeover status");
    assert!(takeover_status.claude);

    let live_taken_over: serde_json::Value =
        read_json_file(&live_path).expect("read taken over live");
    assert_eq!(
        live_taken_over
            .pointer("/env/ANTHROPIC_AUTH_TOKEN")
            .and_then(|value| value.as_str()),
        Some("PROXY_MANAGED")
    );
    let proxy_base_url = live_taken_over
        .pointer("/env/ANTHROPIC_BASE_URL")
        .and_then(|value| value.as_str())
        .expect("proxy base url");
    assert!(proxy_base_url.starts_with("http://127.0.0.1:"));

    let backup = state
        .db
        .get_live_backup("claude")
        .await
        .expect("get claude live backup");
    assert!(backup.is_some());

    state
        .proxy_service
        .stop_with_restore()
        .await
        .expect("stop proxy with restore");

    let status_after_stop = state
        .proxy_service
        .get_status()
        .await
        .expect("get proxy status after stop");
    assert!(!status_after_stop.running);

    let live_restored: serde_json::Value =
        read_json_file(&live_path).expect("read restored live");
    assert_eq!(live_restored, live_before);

    let backup_after_stop = state
        .db
        .get_live_backup("claude")
        .await
        .expect("get backup after stop");
    assert!(backup_after_stop.is_none());

    let app_config = state
        .db
        .get_proxy_config_for_app("claude")
        .await
        .expect("get app proxy config after stop");
    assert!(!app_config.enabled);
}
