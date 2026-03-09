use cc_switch_lib::{AppType, PromptService};

use super::support::{create_empty_state, ensure_test_home, reset_baseline_fs, test_mutex};

#[test]
fn prompt_baseline_import_and_current_file_snapshot_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let home = ensure_test_home().to_path_buf();

    let prompt_path = home.join(".claude").join("CLAUDE.md");
    if let Some(parent) = prompt_path.parent() {
        std::fs::create_dir_all(parent).expect("create prompt dir");
    }
    std::fs::write(&prompt_path, "Always answer with a short status line.\n")
        .expect("seed claude prompt file");

    let state = create_empty_state();
    let imported_id =
        PromptService::import_from_file(&state, AppType::Claude).expect("import prompt file");
    let prompts = PromptService::get_prompts(&state, AppType::Claude).expect("get prompts");
    let current_file =
        PromptService::get_current_file_content(AppType::Claude).expect("get current file");

    let imported = prompts.get(&imported_id).expect("imported prompt exists");
    assert!(!imported.enabled);
    assert!(imported.content.contains("short status line"));
    assert_eq!(current_file.as_deref(), Some(""));
}
