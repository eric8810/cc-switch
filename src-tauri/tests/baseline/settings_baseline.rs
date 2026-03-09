use serde_json::json;

use cc_switch_lib::{get_settings, save_settings, AppSettings};

use super::support::{app_settings_from_json, ensure_test_home, reset_baseline_fs, test_mutex};

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn settings_baseline_save_and_get_snapshot_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let _home = ensure_test_home();

    save_settings(AppSettings {
        language: Some("zh".to_string()),
        show_in_tray: false,
        ..AppSettings::default()
    })
    .await
    .expect("save settings");

    let settings = get_settings().await.expect("get settings");
    assert_eq!(settings.language.as_deref(), Some("zh"));
    assert!(!settings.show_in_tray);
}

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn settings_baseline_terminal_and_visible_apps_snapshot_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let _home = ensure_test_home();

    let settings = app_settings_from_json(json!({
        "language": "en",
        "preferredTerminal": "ghostty",
        "visibleApps": {
            "claude": true,
            "codex": false,
            "gemini": true,
            "opencode": false,
            "openclaw": true
        }
    }));
    save_settings(settings).await.expect("save settings");

    let settings = get_settings().await.expect("get saved settings");
    assert_eq!(settings.preferred_terminal.as_deref(), Some("ghostty"));
    let visible_apps = settings
        .visible_apps
        .expect("visible apps should be stored");
    assert!(visible_apps.claude);
    assert!(!visible_apps.codex);
    assert!(!visible_apps.opencode);
}
