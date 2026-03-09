use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use cc_switch_lib::{update_settings, AppSettings, AppState, AppType, Database, SkillService};

#[path = "../support.rs"]
mod legacy_support;

pub use legacy_support::{ensure_test_home, test_mutex};

pub fn reset_baseline_fs() {
    legacy_support::reset_test_fs();

    let home = ensure_test_home().to_path_buf();
    for relative in [".openclaw", ".config", ".agents"] {
        let path = home.join(relative);
        if path.exists() {
            let _ = fs::remove_dir_all(path);
        }
    }

    for relative in [".zshrc", ".bashrc", ".zprofile", ".profile"] {
        let path = home.join(relative);
        if path.exists() {
            let _ = fs::remove_file(path);
        }
    }
}

pub fn create_empty_state() -> AppState {
    let _ = update_settings(AppSettings::default());
    AppState::new(Arc::new(Database::init().expect("init legacy db")))
}

pub fn test_db_path() -> PathBuf {
    ensure_test_home().join(".cc-switch").join("cc-switch.db")
}

pub fn seed_usage_log(request_id: &str, created_at_secs: i64) {
    let db_path = test_db_path();
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).expect("create db dir");
    }

    let conn = rusqlite::Connection::open(&db_path).expect("open baseline db");
    conn.execute(
        "INSERT INTO proxy_request_logs (
            request_id, provider_id, app_type, model, request_model,
            input_tokens, output_tokens, cache_read_tokens, cache_creation_tokens,
            input_cost_usd, output_cost_usd, cache_read_cost_usd, cache_creation_cost_usd,
            total_cost_usd, latency_ms, first_token_ms, duration_ms, status_code,
            error_message, session_id, provider_type, is_streaming, cost_multiplier, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)",
        rusqlite::params![
            request_id,
            "provider-a",
            "claude",
            "claude-haiku-4-5-20251001",
            Option::<String>::None,
            120_i64,
            80_i64,
            10_i64,
            5_i64,
            "0.000100",
            "0.000200",
            "0.000010",
            "0.000005",
            "0.000315",
            321_i64,
            Option::<i64>::None,
            Option::<i64>::None,
            200_i64,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None,
            1_i64,
            "1.0",
            created_at_secs
        ],
    )
    .expect("insert usage log");
}

pub fn seed_codex_session() -> PathBuf {
    let home = ensure_test_home().to_path_buf();
    let session_id = "11111111-2222-3333-4444-555555555555";
    let session_path = home
        .join(".codex")
        .join("sessions")
        .join("demo-project")
        .join("2026")
        .join("03")
        .join(format!("{session_id}.jsonl"));
    if let Some(parent) = session_path.parent() {
        fs::create_dir_all(parent).expect("create codex session dir");
    }

    let lines = [
        serde_json::json!({
            "type": "session_meta",
            "timestamp": "2026-03-09T10:00:00Z",
            "payload": {
                "id": session_id,
                "cwd": "/tmp/demo-project"
            }
        })
        .to_string(),
        serde_json::json!({
            "type": "response_item",
            "timestamp": "2026-03-09T10:01:00Z",
            "payload": {
                "type": "message",
                "role": "assistant",
                "content": [{ "type": "output_text", "text": "session summary line" }]
            }
        })
        .to_string(),
    ];

    fs::write(&session_path, format!("{}\n{}\n", lines[0], lines[1])).expect("write codex session");
    session_path
}

pub fn exists_or_symlink(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok()
}

pub fn seed_unmanaged_skill(directory: &str) {
    let dir = SkillService::get_app_skills_dir(&AppType::Claude)
        .expect("claude skills dir")
        .join(directory);
    fs::create_dir_all(&dir).expect("create unmanaged skill dir");
    fs::write(
        dir.join("SKILL.md"),
        "---\nname: Demo Skill\ndescription: local import\n---\nbody\n",
    )
    .expect("write skill");
}

pub fn create_skill_zip() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "cc-switch-skill-baseline-{}.zip",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time")
            .as_nanos()
    ));
    let file = fs::File::create(&path).expect("create zip");
    let mut writer = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default();
    writer
        .start_file("SKILL.md", options)
        .expect("start skill file");
    writer
        .write_all(b"---\nname: Zip Skill\ndescription: from zip\n---\n")
        .expect("write skill");
    writer
        .start_file("README.md", options)
        .expect("start readme");
    writer.write_all(b"zip body").expect("write readme");
    writer.finish().expect("finish zip");
    path
}

pub fn app_settings_from_json(value: serde_json::Value) -> AppSettings {
    serde_json::from_value(value).expect("deserialize app settings")
}
