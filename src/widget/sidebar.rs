use iced::widget::{button, column, container, text, Column};
use iced::{Element, Length};

use crate::app::message::Message;
use crate::app::state::Screen;
use crate::theme::{button as btn_style, container as cont_style, SIDEBAR_WIDTH, SPACING_SM, SPACING_XS};

pub fn sidebar(current: &Screen) -> Element<'static, Message> {
    let items = [
        ("Browse", Screen::Browse),
        ("Installed", Screen::Installed),
        ("Updates", Screen::Updates),
    ];

    let buttons: Vec<Element<Message>> = items
        .into_iter()
        .map(|(label, screen)| {
            let is_active = current == &screen;
            button(text(label).size(14))
                .on_press(Message::Navigate(screen))
                .style(if is_active {
                    btn_style::sidebar_active
                } else {
                    btn_style::sidebar
                })
                .width(Length::Fill)
                .padding([SPACING_SM, SPACING_SM])
                .into()
        })
        .collect();

    let nav = Column::with_children(buttons)
        .spacing(SPACING_XS)
        .padding(SPACING_SM);

    let header = container(
        text("Waybar Manager")
            .size(18)
            .color(crate::theme::THEME.text),
    )
    .padding([SPACING_SM * 2.0, SPACING_SM]);

    container(column![header, nav])
        .style(cont_style::sidebar)
        .width(Length::Fixed(SIDEBAR_WIDTH))
        .height(Length::Fill)
        .into()
}
