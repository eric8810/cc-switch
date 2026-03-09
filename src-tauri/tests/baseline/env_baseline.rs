use cc_switch_lib::{check_env_conflicts, delete_env_vars, restore_env_backup};

use super::support::{ensure_test_home, reset_baseline_fs, test_mutex};

fn seed_shell_conflict() -> std::path::PathBuf {
    let home = ensure_test_home().to_path_buf();
    let path = home.join(".zshrc");
    std::fs::write(&path, "export ANTHROPIC_API_KEY=legacy-key\n").expect("seed zshrc");
    path
}

#[test]
fn env_baseline_file_conflict_delete_restore_is_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let path = seed_shell_conflict();

    let conflicts = check_env_conflicts("claude".to_string()).expect("scan env conflicts");
    let file_conflicts: Vec<_> = conflicts
        .into_iter()
        .filter(|item| {
            item.source_type == "file"
                && item
                    .source_path
                    .starts_with(path.to_string_lossy().as_ref())
        })
        .collect();
    assert_eq!(file_conflicts.len(), 1);

    let backup = delete_env_vars(file_conflicts).expect("delete env vars");
    let deleted = std::fs::read_to_string(&path).expect("read deleted file");
    assert!(!deleted.contains("ANTHROPIC_API_KEY"));

    restore_env_backup(backup.backup_path).expect("restore env backup");
    let restored = std::fs::read_to_string(&path).expect("read restored file");
    assert!(restored.contains("ANTHROPIC_API_KEY=legacy-key"));
}
