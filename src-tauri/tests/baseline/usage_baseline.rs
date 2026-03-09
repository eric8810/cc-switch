use super::support::{
    create_empty_state, ensure_test_home, reset_baseline_fs, seed_usage_log, test_mutex,
};

#[test]
fn usage_baseline_summary_handles_seconds_window() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let _home = ensure_test_home();
    let state = create_empty_state();
    seed_usage_log("req-usage-baseline", 1_710_000_000);

    let summary = state
        .db
        .get_usage_summary(Some(1_709_999_000), Some(1_710_001_000))
        .expect("usage summary");

    assert_eq!(summary.total_requests, 1);
    assert_eq!(summary.total_input_tokens, 120);
    assert_eq!(summary.total_output_tokens, 80);
    assert_eq!(summary.total_cache_read_tokens, 10);
    assert_eq!(summary.total_cache_creation_tokens, 5);
    assert_eq!(summary.success_rate, 100.0);
}
