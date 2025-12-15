pub mod message;
pub mod state;

use iced::widget::{column, container, pick_list, row, scrollable, stack, text, text_input, Space};
use iced::{Alignment, Element, Length, Task, Theme};
use iced_aw::Wrap;

use crate::tasks;
use crate::theme::{container as cont_style, SPACING_LG, SPACING_MD, SPACING_SM, THEME};
use crate::widget::{module_card, module_row, notification_toast, sidebar};

pub use message::Message;
pub use state::{App, CategoryFilter, LoadingState, Screen};

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let app = Self {
            loading: LoadingState::Loading,
            ..Default::default()
        };
        (app, tasks::initial_load())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(screen) => {
                self.screen = screen;
                Task::none()
            }

            Message::SearchChanged(query) => {
                self.browse.search_query = query;
                Task::none()
            }

            Message::CategorySelected(filter) => {
                self.browse.selected_category = filter;
                Task::none()
            }

            Message::ModuleClicked(_uuid) => Task::none(),

            Message::InstallModule(_uuid) => Task::none(),

            Message::ToggleModule { uuid, enabled } => {
                self.installed.toggling.insert(uuid.clone());
                tasks::toggle_module(uuid, enabled)
            }

            Message::UninstallModule(uuid) => {
                self.installed.uninstalling.insert(uuid.clone());
                tasks::uninstall_module(uuid)
            }

            Message::OpenPreferences(_uuid) => Task::none(),

            Message::RegistryLoaded(result) => {
                match result {
                    Ok(index) => {
                        self.registry = Some(index);
                        self.loading = LoadingState::Idle;
                    }
                    Err(e) => {
                        self.loading = LoadingState::Failed(e);
                    }
                }
                Task::none()
            }

            Message::InstalledLoaded(result) => {
                match result {
                    Ok(modules) => {
                        self.installed_uuids =
                            modules.iter().map(|m| m.uuid.to_string()).collect();
                        self.installed_modules = modules;
                    }
                    Err(e) => {
                        self.notification = Some(state::Notification {
                            message: format!("Failed to load installed modules: {e}"),
                            kind: state::NotificationKind::Error,
                        });
                    }
                }
                Task::none()
            }

            Message::InstallCompleted(result) => {
                match result {
                    Ok(module) => {
                        self.installed_uuids.insert(module.uuid.to_string());
                        self.installed_modules.push(module);
                        self.notification = Some(state::Notification {
                            message: "Module installed successfully".to_string(),
                            kind: state::NotificationKind::Success,
                        });
                    }
                    Err(e) => {
                        self.notification = Some(state::Notification {
                            message: format!("Installation failed: {e}"),
                            kind: state::NotificationKind::Error,
                        });
                    }
                }
                Task::none()
            }

            Message::ToggleCompleted(result) => {
                match result {
                    Ok(uuid) => {
                        self.installed.toggling.remove(&uuid);
                        if let Some(m) = self
                            .installed_modules
                            .iter_mut()
                            .find(|m| m.uuid.to_string() == uuid)
                        {
                            m.enabled = !m.enabled;
                        }
                    }
                    Err(e) => {
                        self.notification = Some(state::Notification {
                            message: format!("Toggle failed: {e}"),
                            kind: state::NotificationKind::Error,
                        });
                    }
                }
                Task::none()
            }

            Message::UninstallCompleted(result) => {
                match result {
                    Ok(uuid) => {
                        self.installed.uninstalling.remove(&uuid);
                        self.installed_uuids.remove(&uuid);
                        self.installed_modules.retain(|m| m.uuid.to_string() != uuid);
                        self.notification = Some(state::Notification {
                            message: "Module uninstalled".to_string(),
                            kind: state::NotificationKind::Success,
                        });
                    }
                    Err(e) => {
                        self.notification = Some(state::Notification {
                            message: format!("Uninstall failed: {e}"),
                            kind: state::NotificationKind::Error,
                        });
                    }
                }
                Task::none()
            }

            Message::ShowNotification(message, kind) => {
                self.notification = Some(state::Notification { message, kind });
                Task::none()
            }

            Message::DismissNotification => {
                self.notification = None;
                Task::none()
            }

            Message::Tick => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = sidebar(&self.screen);
        let content = match self.screen {
            Screen::Browse => self.view_browse(),
            Screen::Installed => self.view_installed(),
            Screen::Updates => self.view_updates(),
        };

        let main_content = container(content)
            .style(cont_style::page)
            .width(Length::Fill)
            .height(Length::Fill);

        let main_layout = row![sidebar, main_content];

        let notification_overlay: Element<Message> = if let Some(notif) = &self.notification {
            container(notification_toast(notif))
                .width(Length::Fill)
                .padding([SPACING_LG, 0.0])
                .align_x(Alignment::Center)
                .into()
        } else {
            Space::new().width(0.0).height(0.0).into()
        };

        stack![main_layout, notification_overlay].into()
    }

    fn view_browse(&self) -> Element<'_, Message> {
        let search_input = text_input("Search modules...", &self.browse.search_query)
            .on_input(Message::SearchChanged)
            .padding(SPACING_MD)
            .width(Length::Fixed(300.0));

        let category_picker = pick_list(
            CategoryFilter::all(),
            Some(self.browse.selected_category),
            Message::CategorySelected,
        )
        .padding(SPACING_SM);

        let header = container(
            row![
                search_input,
                Space::new().width(Length::Fill),
                category_picker,
            ]
            .spacing(SPACING_LG)
            .align_y(Alignment::Center),
        )
        .padding([SPACING_MD, SPACING_LG]);

        let content: Element<Message> = match &self.loading {
            LoadingState::Loading => container(
                column![
                    text("Loading modules...")
                        .size(18)
                        .color(THEME.text_secondary),
                    text("Fetching registry data")
                        .size(14)
                        .color(THEME.text_muted),
                ]
                .spacing(SPACING_SM)
                .align_x(Alignment::Center),
            )
            .center(Length::Fill)
            .into(),

            LoadingState::Failed(error) => container(
                column![
                    text("Failed to load modules")
                        .size(18)
                        .color(THEME.danger),
                    text(error).size(14).color(THEME.text_muted),
                ]
                .spacing(SPACING_SM)
                .align_x(Alignment::Center),
            )
            .center(Length::Fill)
            .into(),

            LoadingState::Idle => {
                let filtered = self.filtered_modules();
                let cards: Vec<Element<Message>> = filtered
                    .iter()
                    .map(|m| {
                        let uuid = m.uuid.to_string();
                        let is_installed = self.installed_uuids.contains(&uuid);
                        module_card(m, is_installed)
                    })
                    .collect();

                if cards.is_empty() {
                    container(
                        column![
                            text("No modules found")
                                .size(18)
                                .color(THEME.text_secondary),
                            text("Try adjusting your search or category filter")
                                .size(14)
                                .color(THEME.text_muted),
                        ]
                        .spacing(SPACING_SM)
                        .align_x(Alignment::Center),
                    )
                    .center(Length::Fill)
                    .into()
                } else {
                    let grid = Wrap::with_elements(cards)
                        .spacing(SPACING_LG)
                        .line_spacing(SPACING_LG)
                        .align_items(Alignment::Start);

                    scrollable(container(grid).padding(SPACING_LG))
                        .height(Length::Fill)
                        .into()
                }
            }
        };

        column![header, content]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_installed(&self) -> Element<'_, Message> {
        let module_count = self.installed_modules.len();
        let enabled_count = self.installed_modules.iter().filter(|m| m.enabled).count();

        let header = container(
            row![
                column![
                    text("Installed Modules").size(20).color(THEME.text),
                    text(format!(
                        "{} modules ({} enabled)",
                        module_count, enabled_count
                    ))
                    .size(13)
                    .color(THEME.text_secondary),
                ]
                .spacing(SPACING_SM / 2.0),
            ]
            .align_y(Alignment::Center),
        )
        .padding([SPACING_MD, SPACING_LG]);

        let rows: Vec<Element<Message>> = self
            .installed_modules
            .iter()
            .map(|m| {
                let uuid = m.uuid.to_string();
                let is_toggling = self.installed.toggling.contains(&uuid);
                module_row(m, is_toggling)
            })
            .collect();

        let content: Element<Message> = if rows.is_empty() {
            container(
                column![
                    text("No modules installed").size(18).color(THEME.text_secondary),
                    text("Browse modules to find and install new ones")
                        .size(14)
                        .color(THEME.text_muted),
                ]
                .spacing(SPACING_SM)
                .align_x(Alignment::Center),
            )
            .center(Length::Fill)
            .into()
        } else {
            scrollable(
                column(rows)
                    .spacing(SPACING_SM)
                    .padding(SPACING_LG),
            )
            .height(Length::Fill)
            .into()
        };

        column![header, content]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_updates(&self) -> Element<'_, Message> {
        let header = container(
            column![
                text("Updates").size(20).color(THEME.text),
                text("Check for module updates")
                    .size(13)
                    .color(THEME.text_secondary),
            ]
            .spacing(SPACING_SM / 2.0),
        )
        .padding([SPACING_MD, SPACING_LG]);

        let content = container(
            column![
                text("No updates available")
                    .size(18)
                    .color(THEME.text_secondary),
                text("All your modules are up to date")
                    .size(14)
                    .color(THEME.text_muted),
            ]
            .spacing(SPACING_SM)
            .align_x(Alignment::Center),
        )
        .center(Length::Fill);

        column![header, content]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        use iced::event::{self, Event};
        use iced::keyboard;
        use iced::keyboard::key::Named;

        event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(Named::Escape),
                ..
            }) => Some(Message::DismissNotification),
            _ => None,
        })
    }
}
