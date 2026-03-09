use cc_switch_lib::{get_session_messages, list_sessions};

use super::support::{reset_baseline_fs, seed_codex_session, test_mutex};

#[allow(clippy::await_holding_lock)]
#[tokio::test]
async fn session_baseline_codex_scan_and_message_load_are_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let path = seed_codex_session();

    let sessions = list_sessions().await.expect("list sessions");
    let session = sessions
        .iter()
        .find(|item| item.source_path.as_deref() == Some(path.to_string_lossy().as_ref()))
        .expect("seeded codex session");
    assert_eq!(session.provider_id, "codex");
    assert_eq!(session.session_id, "11111111-2222-3333-4444-555555555555");
    assert_eq!(
        session.resume_command.as_deref(),
        Some("codex resume 11111111-2222-3333-4444-555555555555")
    );

    let messages = get_session_messages("codex".to_string(), path.to_string_lossy().to_string())
        .await
        .expect("load session messages");
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].role, "assistant");
    assert!(messages[0].content.contains("session summary line"));
}
