use std::collections::HashSet;
use std::fmt;

use crate::domain::{InstalledModule, ModuleCategory, RegistryIndex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Browse,
    Installed,
    Updates,
}

impl Screen {
    pub fn label(&self) -> &'static str {
        match self {
            Screen::Browse => "Browse",
            Screen::Installed => "Installed",
            Screen::Updates => "Updates",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CategoryFilter(pub Option<ModuleCategory>);

impl CategoryFilter {
    pub fn all() -> Vec<Self> {
        std::iter::once(Self(None))
            .chain(ModuleCategory::all().iter().map(|c| Self(Some(*c))))
            .collect()
    }

    pub fn inner(&self) -> Option<ModuleCategory> {
        self.0
    }
}

impl fmt::Display for CategoryFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(category) => write!(f, "{}", category.display_name()),
            None => write!(f, "All Categories"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BrowseState {
    pub search_query: String,
    pub selected_category: CategoryFilter,
}

#[derive(Debug, Clone, Default)]
pub struct InstalledState {
    pub toggling: HashSet<String>,
    pub uninstalling: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub kind: NotificationKind,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NotificationKind {
    #[default]
    Info,
    Success,
    Error,
}

#[derive(Debug, Clone, Default)]
pub enum LoadingState {
    #[default]
    Idle,
    Loading,
    Failed(String),
}

impl LoadingState {
    pub fn is_loading(&self) -> bool {
        matches!(self, LoadingState::Loading)
    }
}

#[derive(Default)]
pub struct App {
    pub screen: Screen,

    pub registry: Option<RegistryIndex>,
    pub installed_modules: Vec<InstalledModule>,
    pub installed_uuids: HashSet<String>,

    pub browse: BrowseState,
    pub installed: InstalledState,

    pub notification: Option<Notification>,
    pub loading: LoadingState,
}

impl App {
    pub fn filtered_modules(&self) -> Vec<&crate::domain::RegistryModule> {
        let Some(registry) = &self.registry else {
            return Vec::new();
        };

        let query = &self.browse.search_query;
        let category = self.browse.selected_category.inner();

        registry
            .modules
            .iter()
            .filter(|m| {
                let matches_search = query.is_empty() || m.matches_search(query);
                let matches_category =
                    category.is_none() || category.as_ref() == Some(&m.category);
                matches_search && matches_category
            })
            .collect()
    }
}
