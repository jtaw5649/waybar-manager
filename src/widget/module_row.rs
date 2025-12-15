use iced::widget::{button, column, container, row, text, toggler, Space};
use iced::{Alignment, Element, Length};

use crate::app::message::Message;
use crate::domain::InstalledModule;
use crate::theme::{
    button as btn_style, container as cont_style, SPACING_LG, SPACING_MD, SPACING_SM, THEME,
};

pub fn module_row(module: &InstalledModule, is_toggling: bool) -> Element<'static, Message> {
    let uuid = module.uuid.to_string();
    let uuid_toggle = uuid.clone();
    let uuid_uninstall = uuid.clone();
    let name = module.waybar_module_name.clone();
    let enabled = module.enabled;

    let status_text = if enabled {
        text("Enabled").size(12).color(THEME.success)
    } else {
        text("Disabled").size(12).color(THEME.text_muted)
    };

    let toggle_widget: Element<Message> = if is_toggling {
        container(text("...").size(14).color(THEME.text_secondary))
            .padding([SPACING_SM, SPACING_MD])
            .into()
    } else {
        toggler(enabled)
            .on_toggle(move |new_enabled| Message::ToggleModule {
                uuid: uuid_toggle.clone(),
                enabled: new_enabled,
            })
            .size(20.0)
            .into()
    };

    let uninstall_btn = button(text("Uninstall").size(13))
        .on_press(Message::UninstallModule(uuid_uninstall))
        .style(btn_style::secondary)
        .padding([SPACING_SM, SPACING_MD]);

    let info_column = column![
        text(name).size(15).color(THEME.text),
        row![
            text(uuid).size(11).color(THEME.text_muted),
            Space::new().width(SPACING_MD),
            status_text,
        ]
        .spacing(SPACING_SM),
    ]
    .spacing(SPACING_SM / 2.0);

    container(
        row![
            info_column,
            Space::new().width(Length::Fill),
            toggle_widget,
            uninstall_btn,
        ]
        .spacing(SPACING_LG)
        .padding(SPACING_MD)
        .align_y(Alignment::Center),
    )
    .style(cont_style::list_item)
    .width(Length::Fill)
    .into()
}
