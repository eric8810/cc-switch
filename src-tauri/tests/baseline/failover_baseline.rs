use serde_json::json;

use cc_switch_lib::{
    get_auto_failover_enabled_test_hook, set_auto_failover_enabled_test_hook, Provider,
};

use super::support::{create_empty_state, reset_baseline_fs, test_mutex};

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn failover_baseline_queue_round_trip_tracks_available_providers() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let state = create_empty_state();

    let mut primary = Provider::with_id(
        "primary".to_string(),
        "Primary".to_string(),
        json!({ "env": { "ANTHROPIC_AUTH_TOKEN": "key-primary" } }),
        None,
    );
    primary.sort_index = Some(1);

    let mut backup = Provider::with_id(
        "backup".to_string(),
        "Backup".to_string(),
        json!({ "env": { "ANTHROPIC_AUTH_TOKEN": "key-backup" } }),
        None,
    );
    backup.sort_index = Some(2);

    state
        .db
        .save_provider("claude", &primary)
        .expect("save primary provider");
    state
        .db
        .save_provider("claude", &backup)
        .expect("save backup provider");

    let available_before = state
        .db
        .get_available_providers_for_failover("claude")
        .expect("available providers before");
    assert_eq!(available_before.len(), 2);

    state
        .db
        .add_to_failover_queue("claude", "primary")
        .expect("add primary to queue");

    let queue = state
        .db
        .get_failover_queue("claude")
        .expect("failover queue after add");
    assert_eq!(queue.len(), 1);
    assert_eq!(queue[0].provider_id, "primary");

    let available_after = state
        .db
        .get_available_providers_for_failover("claude")
        .expect("available providers after add");
    assert_eq!(available_after.len(), 1);
    assert_eq!(available_after[0].id, "backup");

    state
        .db
        .remove_from_failover_queue("claude", "primary")
        .expect("remove primary from queue");

    let queue_after_remove = state
        .db
        .get_failover_queue("claude")
        .expect("queue after remove");
    assert!(queue_after_remove.is_empty());
}

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn failover_baseline_enable_auto_failover_auto_seeds_queue_from_current_provider() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let state = create_empty_state();

    let provider = Provider::with_id(
        "primary".to_string(),
        "Primary".to_string(),
        json!({ "env": { "ANTHROPIC_AUTH_TOKEN": "key-primary" } }),
        None,
    );
    state
        .db
        .save_provider("claude", &provider)
        .expect("save current provider");
    state
        .db
        .set_current_provider("claude", "primary")
        .expect("set current provider");

    let switched_to = set_auto_failover_enabled_test_hook(&state, "claude", true)
        .await
        .expect("enable auto failover");
    assert_eq!(switched_to.as_deref(), Some("primary"));

    let queue = state
        .db
        .get_failover_queue("claude")
        .expect("failover queue");
    assert_eq!(queue.len(), 1);
    assert_eq!(queue[0].provider_id, "primary");

    let auto_enabled = get_auto_failover_enabled_test_hook(&state, "claude")
        .await
        .expect("read auto failover enabled");
    assert!(auto_enabled);
}

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn failover_baseline_enable_switches_to_queue_head_and_disable_keeps_queue() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let state = create_empty_state();

    let mut primary = Provider::with_id(
        "primary".to_string(),
        "Primary".to_string(),
        json!({ "env": { "ANTHROPIC_AUTH_TOKEN": "key-primary" } }),
        None,
    );
    primary.sort_index = Some(1);

    let mut secondary = Provider::with_id(
        "secondary".to_string(),
        "Secondary".to_string(),
        json!({ "env": { "ANTHROPIC_AUTH_TOKEN": "key-secondary" } }),
        None,
    );
    secondary.sort_index = Some(2);

    state
        .db
        .save_provider("claude", &primary)
        .expect("save primary");
    state
        .db
        .save_provider("claude", &secondary)
        .expect("save secondary");
    state
        .db
        .set_current_provider("claude", "secondary")
        .expect("set current secondary");
    state
        .db
        .add_to_failover_queue("claude", "primary")
        .expect("add queue head");

    let switched_to = set_auto_failover_enabled_test_hook(&state, "claude", true)
        .await
        .expect("enable auto failover");
    assert_eq!(switched_to.as_deref(), Some("primary"));

    let current = state
        .db
        .get_current_provider("claude")
        .expect("read current provider");
    assert_eq!(current.as_deref(), Some("primary"));

    set_auto_failover_enabled_test_hook(&state, "claude", false)
        .await
        .expect("disable auto failover");
    let auto_enabled = get_auto_failover_enabled_test_hook(&state, "claude")
        .await
        .expect("read auto failover disabled");
    assert!(!auto_enabled);

    let queue_after_disable = state
        .db
        .get_failover_queue("claude")
        .expect("queue after disable");
    assert_eq!(queue_after_disable.len(), 1);
    assert_eq!(queue_after_disable[0].provider_id, "primary");
}
