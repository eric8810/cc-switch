use cc_switch_lib::{AppType, SkillService};

use super::support::{
    create_empty_state, create_skill_zip, ensure_test_home, exists_or_symlink, reset_baseline_fs,
    seed_unmanaged_skill, test_mutex,
};

#[test]
fn skill_baseline_import_toggle_and_zip_install_are_stable() {
    let _guard = test_mutex().lock().unwrap_or_else(|err| err.into_inner());
    reset_baseline_fs();
    let _home = ensure_test_home();
    seed_unmanaged_skill("demo-skill");

    let state = create_empty_state();
    let unmanaged = SkillService::scan_unmanaged(&state.db).expect("scan unmanaged");
    assert_eq!(unmanaged.len(), 1);
    assert_eq!(unmanaged[0].directory, "demo-skill");
    assert_eq!(unmanaged[0].found_in, vec!["claude".to_string()]);

    let imported =
        SkillService::import_from_apps(&state.db, vec!["demo-skill".to_string()]).expect("import");
    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].id, "local:demo-skill");
    assert!(imported[0].apps.claude);

    let ssot_dir = SkillService::get_ssot_dir()
        .expect("ssot dir")
        .join("demo-skill");
    let app_dir = SkillService::get_app_skills_dir(&AppType::Claude)
        .expect("claude app dir")
        .join("demo-skill");
    assert!(exists_or_symlink(&ssot_dir));
    assert!(exists_or_symlink(&app_dir));

    SkillService::toggle_app(&state.db, "local:demo-skill", &AppType::Claude, false)
        .expect("disable skill for claude");
    let installed = SkillService::get_all_installed(&state.db).expect("get installed skills");
    assert_eq!(installed.len(), 1);
    assert!(!installed[0].apps.claude);
    assert!(exists_or_symlink(&ssot_dir));
    assert!(!exists_or_symlink(&app_dir));

    let zip_path = create_skill_zip();
    let zipped = SkillService::install_from_zip(&state.db, &zip_path, &AppType::Claude)
        .expect("install zip");
    assert_eq!(zipped.len(), 1);
    assert!(zipped[0].apps.claude);
    assert!(exists_or_symlink(
        &SkillService::get_ssot_dir()
            .expect("ssot dir")
            .join(&zipped[0].directory)
    ));
    let _ = std::fs::remove_file(zip_path);
}
