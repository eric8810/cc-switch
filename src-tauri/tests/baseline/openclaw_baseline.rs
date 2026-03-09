use serde_json::Value;

use cc_switch_lib::{
    get_openclaw_agents_defaults, get_openclaw_default_model, get_openclaw_env,
    get_openclaw_tools, set_openclaw_agents_defaults, set_openclaw_default_model,
    set_openclaw_env, set_openclaw_tools,
};

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

#[test]
fn openclaw_baseline_default_model_round_trip_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();

    let model = get_openclaw_default_model()
        .expect("get openclaw default model")
        .unwrap_or_else(|| {
            serde_json::from_value(serde_json::json!({
                "primary": "provider-a/model-primary",
                "fallbacks": ["provider-b/model-fallback"]
            }))
            .expect("deserialize default model")
        });
    set_openclaw_default_model(model.clone()).expect("set default model");

    let stored = get_openclaw_default_model()
        .expect("get stored default model")
        .expect("default model exists");
    let stored_value = serde_json::to_value(&stored).expect("serialize stored model");
    assert_eq!(
        stored_value.get("primary").and_then(|value| value.as_str()),
        Some("provider-a/model-primary")
    );
}

#[test]
fn openclaw_baseline_agents_defaults_and_catalog_round_trip_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();

    let defaults = get_openclaw_agents_defaults()
        .expect("get agents defaults")
        .unwrap_or_else(|| {
            serde_json::from_value(serde_json::json!({
                "model": {
                    "primary": "provider-a/model-primary",
                    "fallbacks": ["provider-b/model-fallback"]
                },
                "models": {
                    "provider-a/model-primary": { "alias": "primary" }
                }
            }))
            .expect("deserialize agents defaults")
        });
    set_openclaw_agents_defaults(defaults.clone()).expect("set agents defaults");

    let stored_defaults = get_openclaw_agents_defaults()
        .expect("get stored agents defaults")
        .expect("agents defaults exist");
    let stored_defaults_value =
        serde_json::to_value(&stored_defaults).expect("serialize agents defaults");
    assert_eq!(
        stored_defaults_value
            .pointer("/model/primary")
            .and_then(|value| value.as_str()),
        Some("provider-a/model-primary")
    );
    assert!(
        stored_defaults_value.pointer("/models").is_some(),
        "agents.defaults should preserve models allowlist"
    );
}

#[test]
fn openclaw_baseline_tools_round_trip_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();

    let mut tools = get_openclaw_tools().expect("get default tools");
    tools.profile = Some("strict".to_string());
    tools.allow = vec!["Read(*)".to_string()];
    tools.deny = vec!["Shell(*)".to_string()];
    set_openclaw_tools(tools.clone()).expect("set tools config");

    let stored = get_openclaw_tools().expect("get stored tools config");
    assert_eq!(stored.profile.as_deref(), Some("strict"));
    assert_eq!(stored.allow, vec!["Read(*)"]);
    assert_eq!(stored.deny, vec!["Shell(*)"]);
}
