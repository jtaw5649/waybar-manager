use crate::app::state::{CategoryFilter, NotificationKind, Screen};
use crate::domain::{InstalledModule, RegistryIndex};

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Screen),

    SearchChanged(String),
    CategorySelected(CategoryFilter),
    ModuleClicked(String),
    InstallModule(String),

    ToggleModule { uuid: String, enabled: bool },
    UninstallModule(String),
    OpenPreferences(String),

    RegistryLoaded(Result<RegistryIndex, String>),
    InstalledLoaded(Result<Vec<InstalledModule>, String>),
    InstallCompleted(Result<InstalledModule, String>),
    ToggleCompleted(Result<String, String>),
    UninstallCompleted(Result<String, String>),

    ShowNotification(String, NotificationKind),
    DismissNotification,
    Tick,
}
