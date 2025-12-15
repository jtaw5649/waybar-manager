use iced::Task;

use crate::app::Message;
use crate::domain::{InstalledModule, RegistryIndex};
use crate::services::paths;

const REGISTRY_URL: &str = "https://raw.githubusercontent.com/waybar-modules/registry/main/index.json";

pub fn initial_load() -> Task<Message> {
    Task::batch([load_installed(), load_registry()])
}

pub fn load_registry() -> Task<Message> {
    Task::perform(fetch_registry_async(), Message::RegistryLoaded)
}

pub fn load_installed() -> Task<Message> {
    Task::perform(load_installed_async(), Message::InstalledLoaded)
}

pub fn toggle_module(uuid: String, enabled: bool) -> Task<Message> {
    Task::perform(
        toggle_module_async(uuid, enabled),
        Message::ToggleCompleted,
    )
}

pub fn uninstall_module(uuid: String) -> Task<Message> {
    Task::perform(uninstall_module_async(uuid), Message::UninstallCompleted)
}

async fn fetch_registry_async() -> Result<RegistryIndex, String> {
    let cache_path = paths::registry_cache_path();

    if let Ok(content) = tokio::fs::read_to_string(&cache_path).await
        && let Ok(index) = serde_json::from_str::<RegistryIndex>(&content)
    {
        tracing::info!("Loaded registry from cache ({} modules)", index.modules.len());
        return Ok(index);
    }

    tracing::info!("Fetching registry from {}", REGISTRY_URL);
    let response = reqwest::get(REGISTRY_URL)
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    let index: RegistryIndex = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {e}"))?;

    if let Some(parent) = cache_path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    if let Ok(content) = serde_json::to_string_pretty(&index) {
        let _ = tokio::fs::write(&cache_path, content).await;
    }

    tracing::info!("Fetched {} modules from registry", index.modules.len());
    Ok(index)
}

async fn load_installed_async() -> Result<Vec<InstalledModule>, String> {
    let state_path = paths::data_dir().join("installed.json");

    if !state_path.exists() {
        tracing::debug!("No installed modules file found");
        return Ok(Vec::new());
    }

    let content = tokio::fs::read_to_string(&state_path)
        .await
        .map_err(|e| format!("Failed to read installed modules: {e}"))?;

    let modules: Vec<InstalledModule> =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse installed modules: {e}"))?;

    tracing::info!("Loaded {} installed modules", modules.len());
    Ok(modules)
}

async fn toggle_module_async(uuid: String, enabled: bool) -> Result<String, String> {
    let state_path = paths::data_dir().join("installed.json");

    let content = tokio::fs::read_to_string(&state_path)
        .await
        .map_err(|e| format!("Failed to read state: {e}"))?;

    let mut modules: Vec<InstalledModule> =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse state: {e}"))?;

    let module = modules
        .iter_mut()
        .find(|m| m.uuid.to_string() == uuid)
        .ok_or_else(|| format!("Module not found: {uuid}"))?;

    module.enabled = enabled;

    let new_content =
        serde_json::to_string_pretty(&modules).map_err(|e| format!("Failed to serialize: {e}"))?;

    tokio::fs::write(&state_path, new_content)
        .await
        .map_err(|e| format!("Failed to save state: {e}"))?;

    tracing::info!("Module {} {}", uuid, if enabled { "enabled" } else { "disabled" });
    Ok(uuid)
}

async fn uninstall_module_async(uuid: String) -> Result<String, String> {
    let state_path = paths::data_dir().join("installed.json");
    let install_path = paths::module_install_path(&uuid);

    if install_path.exists() {
        tokio::fs::remove_dir_all(&install_path)
            .await
            .map_err(|e| format!("Failed to remove module files: {e}"))?;
    }

    let content = tokio::fs::read_to_string(&state_path)
        .await
        .map_err(|e| format!("Failed to read state: {e}"))?;

    let mut modules: Vec<InstalledModule> =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse state: {e}"))?;

    modules.retain(|m| m.uuid.to_string() != uuid);

    let new_content =
        serde_json::to_string_pretty(&modules).map_err(|e| format!("Failed to serialize: {e}"))?;

    tokio::fs::write(&state_path, new_content)
        .await
        .map_err(|e| format!("Failed to save state: {e}"))?;

    tracing::info!("Uninstalled module {}", uuid);
    Ok(uuid)
}
