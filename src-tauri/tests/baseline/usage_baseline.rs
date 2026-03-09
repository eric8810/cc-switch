use serde_json::json;

use cc_switch_lib::{
    delete_model_pricing_test_hook, get_model_pricing_test_hook, get_request_detail_test_hook,
    update_model_pricing_test_hook, Provider,
};

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

#[test]
fn usage_baseline_request_detail_resolves_provider_name() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let _home = ensure_test_home();
    let state = create_empty_state();

    let provider = Provider::with_id(
        "provider-a".to_string(),
        "Provider Alpha".to_string(),
        json!({ "env": { "ANTHROPIC_AUTH_TOKEN": "alpha-key" } }),
        None,
    );
    state
        .db
        .save_provider("claude", &provider)
        .expect("save provider for detail");
    seed_usage_log("req-usage-detail", 1_710_000_000);

    let detail = get_request_detail_test_hook(&state, "req-usage-detail")
        .expect("get request detail")
        .expect("request detail exists");

    assert_eq!(detail.provider_id, "provider-a");
    assert_eq!(detail.provider_name.as_deref(), Some("Provider Alpha"));
    assert_eq!(detail.model, "claude-haiku-4-5-20251001");
    assert_eq!(detail.status_code, 200);
}

#[test]
fn usage_baseline_model_pricing_crud_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let state = create_empty_state();

    let seeded = get_model_pricing_test_hook(&state).expect("get seeded pricing");
    assert!(
        !seeded.is_empty(),
        "model_pricing should be auto-seeded for baseline tests"
    );

    update_model_pricing_test_hook(
        &state,
        "baseline-model",
        "Baseline Model",
        "1.23",
        "4.56",
        "0.12",
        "0.34",
    )
    .expect("upsert pricing");

    let after_upsert = get_model_pricing_test_hook(&state).expect("get pricing after upsert");
    let inserted = after_upsert
        .iter()
        .find(|item| item.model_id == "baseline-model")
        .expect("inserted pricing row");
    assert_eq!(inserted.display_name, "Baseline Model");
    assert_eq!(inserted.input_cost_per_million, "1.23");
    assert_eq!(inserted.output_cost_per_million, "4.56");

    delete_model_pricing_test_hook(&state, "baseline-model").expect("delete pricing");

    let after_delete = get_model_pricing_test_hook(&state).expect("get pricing after delete");
    assert!(
        after_delete
            .iter()
            .all(|item| item.model_id != "baseline-model"),
        "custom pricing row should be removed"
    );
}
