use serde_json::Value;

use cc_switch_lib::{get_openclaw_env, set_openclaw_env};

use super::support::{reset_baseline_fs, test_mutex};

#[test]
fn openclaw_baseline_env_round_trip_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();

    let mut env = get_openclaw_env().expect("get default openclaw env");
    env.vars.insert(
        "ANTHROPIC_API_KEY".to_string(),
        Value::String("test-key".to_string()),
    );
    set_openclaw_env(env.clone()).expect("set openclaw env");

    let stored = get_openclaw_env().expect("get stored openclaw env");
    assert_eq!(
        stored.vars.get("ANTHROPIC_API_KEY"),
        Some(&Value::String("test-key".to_string()))
    );
}
