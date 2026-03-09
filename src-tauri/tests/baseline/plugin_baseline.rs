use cc_switch_lib::{
    apply_claude_onboarding_skip, apply_claude_plugin_config, get_claude_plugin_status,
    is_claude_plugin_applied, read_claude_plugin_config,
};

use super::support::{ensure_test_home, reset_baseline_fs, test_mutex};

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn plugin_baseline_apply_and_onboarding_are_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let home = ensure_test_home().to_path_buf();

    let initial = get_claude_plugin_status().await.expect("get plugin status");
    assert!(!initial.exists);

    assert!(
        apply_claude_plugin_config(false)
            .await
            .expect("apply plugin config"),
        "first managed apply should modify config"
    );
    assert!(is_claude_plugin_applied()
        .await
        .expect("plugin should be applied"));

    let config = read_claude_plugin_config()
        .await
        .expect("read plugin config")
        .expect("plugin config should exist");
    assert!(config.contains("\"primaryApiKey\": \"any\""));

    assert!(
        apply_claude_onboarding_skip()
            .await
            .expect("apply onboarding skip"),
        "onboarding flag should be written"
    );
    let onboarding = std::fs::read_to_string(home.join(".claude.json")).expect("read onboarding");
    assert!(onboarding.contains("\"hasCompletedOnboarding\": true"));
}
