use iced::widget::{button, column, container, text};
use iced::{Element, Length};

use crate::app::message::Message;
use crate::domain::RegistryModule;
use crate::theme::{
    button as btn_style, container as cont_style, CARD_WIDTH, SPACING_MD, SPACING_SM, THEME,
};

pub fn module_card(module: &RegistryModule, is_installed: bool) -> Element<'static, Message> {
    let uuid = module.uuid.to_string();
    let name = module.name.clone();
    let author = module.author.clone();
    let category = module.category.display_name().to_string();

    let install_section: Element<Message> = if is_installed {
        text("Installed").size(12).color(THEME.success).into()
    } else {
        button(text("Install").size(12))
            .on_press(Message::InstallModule(uuid.clone()))
            .style(btn_style::primary)
            .padding([SPACING_SM, SPACING_MD])
            .into()
    };

    container(
        column![
            text(name).size(16).color(THEME.text),
            text(format!("by {author}"))
                .size(12)
                .color(THEME.text_secondary),
            text(category).size(10).color(THEME.text_muted),
            install_section,
        ]
        .spacing(SPACING_SM)
        .padding(SPACING_MD),
    )
    .style(cont_style::card)
    .width(Length::Fixed(CARD_WIDTH))
    .into()
}
