use tauri::State;

use crate::services::omo::{OmoLocalFileData, SLIM, STANDARD};
use crate::services::OmoService;
use crate::store::AppState;

#[tauri::command]
pub async fn read_omo_local_file() -> Result<OmoLocalFileData, String> {
    OmoService::read_local_file(&STANDARD).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_current_omo_provider_id(state: State<'_, AppState>) -> Result<String, String> {
    let provider = state
        .db
        .get_current_omo_provider("opencode", "omo")
        .map_err(|e| e.to_string())?;
    Ok(provider.map(|p| p.id).unwrap_or_default())
}

fn get_current_omo_provider_id_internal(
    state: &AppState,
    category: &str,
) -> Result<String, String> {
    let provider = state
        .db
        .get_current_omo_provider("opencode", category)
        .map_err(|e| e.to_string())?;
    Ok(provider.map(|p| p.id).unwrap_or_default())
}

fn disable_current_omo_internal(state: &AppState, category: &str) -> Result<(), String> {
    let providers = state
        .db
        .get_all_providers("opencode")
        .map_err(|e| e.to_string())?;
    for (id, p) in &providers {
        if p.category.as_deref() == Some(category) {
            state
                .db
                .clear_omo_provider_current("opencode", id, category)
                .map_err(|e| e.to_string())?;
        }
    }
    let variant = if category == STANDARD.category {
        &STANDARD
    } else {
        &SLIM
    };
    OmoService::delete_config_file(variant).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(not(feature = "test-hooks"), doc(hidden))]
pub fn get_current_omo_provider_id_test_hook(
    state: &AppState,
    category: &str,
) -> Result<String, String> {
    get_current_omo_provider_id_internal(state, category)
}

#[cfg_attr(not(feature = "test-hooks"), doc(hidden))]
pub fn disable_current_omo_test_hook(state: &AppState, category: &str) -> Result<(), String> {
    disable_current_omo_internal(state, category)
}

#[tauri::command]
pub async fn disable_current_omo(state: State<'_, AppState>) -> Result<(), String> {
    disable_current_omo_internal(&state, STANDARD.category)
}

// ── OMO Slim commands ───────────────────────────────────────

#[tauri::command]
pub async fn read_omo_slim_local_file() -> Result<OmoLocalFileData, String> {
    OmoService::read_local_file(&SLIM).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_current_omo_slim_provider_id(
    state: State<'_, AppState>,
) -> Result<String, String> {
    get_current_omo_provider_id_internal(&state, SLIM.category)
}

#[tauri::command]
pub async fn disable_current_omo_slim(state: State<'_, AppState>) -> Result<(), String> {
    disable_current_omo_internal(&state, SLIM.category)
}
