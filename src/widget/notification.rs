use iced::widget::{button, container, row, text, Space};
use iced::{Alignment, Element, Length};

use crate::app::message::Message;
use crate::app::state::{Notification, NotificationKind};
use crate::theme::{container as cont_style, SPACING_MD, SPACING_SM, THEME};

pub fn notification_toast(notif: &Notification) -> Element<'static, Message> {
    let icon = match notif.kind {
        NotificationKind::Success => "✓",
        NotificationKind::Error => "✕",
        NotificationKind::Info => "ℹ",
    };

    let icon_color = match notif.kind {
        NotificationKind::Success => THEME.success,
        NotificationKind::Error => THEME.danger,
        NotificationKind::Info => THEME.primary,
    };

    let message = notif.message.clone();

    let dismiss_btn = button(text("×").size(16).color(THEME.text_secondary))
        .on_press(Message::DismissNotification)
        .padding(SPACING_SM)
        .style(crate::theme::button::ghost);

    let content = row![
        text(icon).size(16).color(icon_color),
        text(message).size(14).color(THEME.text),
        Space::new().width(Length::Fill),
        dismiss_btn,
    ]
    .spacing(SPACING_MD)
    .align_y(Alignment::Center)
    .padding(SPACING_MD);

    let style = match notif.kind {
        NotificationKind::Success => cont_style::notification_success,
        NotificationKind::Error => cont_style::notification_error,
        NotificationKind::Info => cont_style::notification_info,
    };

    container(content)
        .style(style)
        .width(Length::Fixed(400.0))
        .into()
}
