use serde_json::json;

use cc_switch_lib::{
    delete_universal_provider_test_hook, get_universal_provider_test_hook,
    get_universal_providers_test_hook, sync_universal_provider_test_hook,
    upsert_universal_provider_test_hook,
};

use super::support::{create_empty_state, reset_baseline_fs, test_mutex};

#[test]
fn universal_provider_baseline_round_trip_and_sync_are_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let state = create_empty_state();

    let upserted = upsert_universal_provider_test_hook(
        &state,
        serde_json::from_value(json!({
            "id": "universal-1",
            "name": "Universal One",
            "providerType": "newapi",
            "apps": {
                "claude": true,
                "codex": true,
                "gemini": false
            },
            "baseUrl": "https://api.example.com",
            "apiKey": "api-key-1",
            "notes": "baseline",
            "meta": {
                "pricingModelSource": "request"
            }
        }))
        .expect("deserialize universal provider"),
    )
    .expect("upsert universal provider");
    assert!(upserted);

    let fetched = get_universal_provider_test_hook(&state, "universal-1")
        .expect("get universal provider")
        .expect("universal provider exists");
    assert_eq!(fetched.name, "Universal One");
    assert!(fetched.apps.claude);
    assert!(fetched.apps.codex);

    let listed = get_universal_providers_test_hook(&state).expect("list universal providers");
    assert_eq!(listed.len(), 1);
    assert!(listed.contains_key("universal-1"));

    let synced = sync_universal_provider_test_hook(&state, "universal-1")
        .expect("sync universal provider");
    assert!(synced);

    let claude_provider = state
        .db
        .get_provider_by_id("universal-claude-universal-1", "claude")
        .expect("get synced claude provider")
        .expect("synced claude provider exists");
    assert_eq!(
        claude_provider
            .settings_config
            .pointer("/env/ANTHROPIC_MODEL")
            .and_then(|value| value.as_str()),
        Some("claude-sonnet-4-20250514")
    );

    let codex_provider = state
        .db
        .get_provider_by_id("universal-codex-universal-1", "codex")
        .expect("get synced codex provider")
        .expect("synced codex provider exists");
    assert_eq!(
        codex_provider
            .settings_config
            .pointer("/auth/OPENAI_API_KEY")
            .and_then(|value| value.as_str()),
        Some("api-key-1")
    );
    let codex_config = codex_provider
        .settings_config
        .pointer("/config")
        .and_then(|value| value.as_str())
        .expect("codex config toml");
    assert!(codex_config.contains("base_url = \"https://api.example.com/v1\""));

    let deleted = delete_universal_provider_test_hook(&state, "universal-1")
        .expect("delete universal provider");
    assert!(deleted);

    let listed_after_delete =
        get_universal_providers_test_hook(&state).expect("list providers after delete");
    assert!(listed_after_delete.is_empty());
    assert!(
        state
            .db
            .get_provider_by_id("universal-claude-universal-1", "claude")
            .expect("check claude provider after delete")
            .is_none()
    );
    assert!(
        state
            .db
            .get_provider_by_id("universal-codex-universal-1", "codex")
            .expect("check codex provider after delete")
            .is_none()
    );
}
