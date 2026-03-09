use serde_json::json;

use cc_switch_lib::{get_settings, webdav_sync_save_settings};

use super::support::{app_settings_from_json, ensure_test_home, reset_baseline_fs, test_mutex};

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn webdav_baseline_save_settings_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let _home = ensure_test_home();

    let seed = app_settings_from_json(json!({
        "webdavSync": {
            "enabled": true,
            "baseUrl": "https://dav.example.com",
            "username": "alice",
            "password": "secret",
            "remoteRoot": "cc-switch/device"
        }
    }));
    let webdav = seed.webdav_sync.expect("webdav payload");

    let result = webdav_sync_save_settings(webdav, Some(true))
        .await
        .expect("save webdav settings");
    assert_eq!(result["success"], json!(true));

    let stored = get_settings()
        .await
        .expect("get settings")
        .webdav_sync
        .expect("stored webdav settings");
    assert!(stored.enabled);
    assert_eq!(stored.base_url, "https://dav.example.com");
    assert_eq!(stored.username, "alice");
    assert_eq!(stored.remote_root, "cc-switch/device");
}
