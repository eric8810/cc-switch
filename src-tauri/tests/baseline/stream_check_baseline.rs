use super::support::{create_empty_state, ensure_test_home, reset_baseline_fs, test_mutex};

#[test]
fn stream_check_baseline_save_and_get_config_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let _home = ensure_test_home();

    let state = create_empty_state();
    let mut config = state
        .db
        .get_stream_check_config()
        .expect("default stream config");
    config.timeout_secs = 12;
    config.max_retries = 4;
    config.degraded_threshold_ms = 3456;
    config.claude_model = "claude-test".to_string();
    config.codex_model = "codex-test".to_string();
    config.gemini_model = "gemini-test".to_string();
    config.test_prompt = "ping".to_string();

    state
        .db
        .save_stream_check_config(&config)
        .expect("save stream config");
    let stored = state
        .db
        .get_stream_check_config()
        .expect("get stored stream config");

    assert_eq!(stored.timeout_secs, 12);
    assert_eq!(stored.max_retries, 4);
    assert_eq!(stored.test_prompt, "ping");
}
