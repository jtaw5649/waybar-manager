use iced::widget::button;
use iced::{Background, Border, Theme};

use super::{RADIUS_MD, RADIUS_SM, THEME};

pub fn primary(_theme: &Theme, status: button::Status) -> button::Style {
    let (background, text) = match status {
        button::Status::Active => (THEME.primary, THEME.text),
        button::Status::Hovered => (THEME.primary_hover, THEME.text),
        button::Status::Pressed => (THEME.primary, THEME.text),
        button::Status::Disabled => (THEME.surface, THEME.text_muted),
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: text,
        border: Border {
            color: background,
            width: 0.0,
            radius: RADIUS_MD.into(),
        },
        ..Default::default()
    }
}

pub fn secondary(_theme: &Theme, status: button::Status) -> button::Style {
    let (background, border_color) = match status {
        button::Status::Active => (THEME.surface, THEME.border),
        button::Status::Hovered => (THEME.surface_hover, THEME.primary),
        button::Status::Pressed => (THEME.surface, THEME.primary),
        button::Status::Disabled => (THEME.surface, THEME.border),
    };

    let text = match status {
        button::Status::Disabled => THEME.text_muted,
        _ => THEME.text,
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: text,
        border: Border {
            color: border_color,
            width: 1.0,
            radius: RADIUS_MD.into(),
        },
        ..Default::default()
    }
}

pub fn danger(_theme: &Theme, status: button::Status) -> button::Style {
    let (background, text) = match status {
        button::Status::Active => (THEME.danger, THEME.text),
        button::Status::Hovered => (THEME.danger_hover, THEME.text),
        button::Status::Pressed => (THEME.danger, THEME.text),
        button::Status::Disabled => (THEME.surface, THEME.text_muted),
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: text,
        border: Border {
            color: background,
            width: 0.0,
            radius: RADIUS_MD.into(),
        },
        ..Default::default()
    }
}

pub fn sidebar(_theme: &Theme, status: button::Status) -> button::Style {
    let (background, text) = match status {
        button::Status::Active => (iced::Color::TRANSPARENT, THEME.text_secondary),
        button::Status::Hovered => (THEME.surface, THEME.text),
        button::Status::Pressed => (THEME.surface, THEME.text),
        button::Status::Disabled => (iced::Color::TRANSPARENT, THEME.text_muted),
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: text,
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: RADIUS_SM.into(),
        },
        ..Default::default()
    }
}

pub fn sidebar_active(_theme: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Hovered => THEME.primary_hover,
        _ => THEME.primary,
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: THEME.text,
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: RADIUS_SM.into(),
        },
        ..Default::default()
    }
}

pub fn ghost(_theme: &Theme, status: button::Status) -> button::Style {
    let (background, text) = match status {
        button::Status::Active => (iced::Color::TRANSPARENT, THEME.text_secondary),
        button::Status::Hovered => (THEME.surface, THEME.text),
        button::Status::Pressed => (THEME.surface_hover, THEME.text),
        button::Status::Disabled => (iced::Color::TRANSPARENT, THEME.text_muted),
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: text,
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: RADIUS_SM.into(),
        },
        ..Default::default()
    }
}
