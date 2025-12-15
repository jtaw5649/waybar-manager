use iced::widget::container;
use iced::{Background, Border, Theme};

use super::{RADIUS_LG, RADIUS_MD, THEME};

pub fn sidebar(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.sidebar_bg)),
        border: Border {
            color: THEME.border,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.surface)),
        border: Border {
            color: THEME.border,
            width: 1.0,
            radius: RADIUS_LG.into(),
        },
        ..Default::default()
    }
}

pub fn card_hover(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.surface_hover)),
        border: Border {
            color: THEME.primary,
            width: 1.0,
            radius: RADIUS_LG.into(),
        },
        ..Default::default()
    }
}

pub fn list_item(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.surface)),
        border: Border {
            color: THEME.border,
            width: 1.0,
            radius: RADIUS_MD.into(),
        },
        ..Default::default()
    }
}

pub fn page(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.background)),
        border: Border::default(),
        ..Default::default()
    }
}

pub fn notification_success(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.success)),
        border: Border {
            color: THEME.success,
            width: 0.0,
            radius: RADIUS_MD.into(),
        },
        ..Default::default()
    }
}

pub fn notification_error(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.danger)),
        border: Border {
            color: THEME.danger,
            width: 0.0,
            radius: RADIUS_MD.into(),
        },
        ..Default::default()
    }
}

pub fn notification_info(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.surface)),
        border: Border {
            color: THEME.primary,
            width: 1.0,
            radius: RADIUS_MD.into(),
        },
        ..Default::default()
    }
}
