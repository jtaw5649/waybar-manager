pub mod button;
pub mod container;

use iced::Color;

#[derive(Debug, Clone, Copy)]
pub struct AppTheme {
    pub background: Color,
    pub surface: Color,
    pub surface_hover: Color,
    pub primary: Color,
    pub primary_hover: Color,
    pub secondary: Color,
    pub text: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub success: Color,
    pub danger: Color,
    pub danger_hover: Color,
    pub border: Color,
    pub sidebar_bg: Color,
}

impl Default for AppTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl AppTheme {
    pub const fn dark() -> Self {
        Self {
            background: Color::from_rgb(0.094, 0.094, 0.110),       // #18181c
            surface: Color::from_rgb(0.141, 0.141, 0.165),          // #24242a
            surface_hover: Color::from_rgb(0.180, 0.180, 0.210),    // #2e2e36
            primary: Color::from_rgb(0.388, 0.400, 0.945),          // #6366f1
            primary_hover: Color::from_rgb(0.488, 0.500, 0.980),    // #7c7ffa
            secondary: Color::from_rgb(0.545, 0.361, 0.965),        // #8b5cf6
            text: Color::from_rgb(0.980, 0.980, 0.980),             // #fafafa
            text_secondary: Color::from_rgb(0.631, 0.631, 0.667),   // #a1a1aa
            text_muted: Color::from_rgb(0.447, 0.447, 0.490),       // #72727d
            success: Color::from_rgb(0.133, 0.773, 0.369),          // #22c55e
            danger: Color::from_rgb(0.937, 0.267, 0.267),           // #ef4444
            danger_hover: Color::from_rgb(0.980, 0.380, 0.380),     // #fa6161
            border: Color::from_rgb(0.247, 0.247, 0.275),           // #3f3f46
            sidebar_bg: Color::from_rgb(0.118, 0.118, 0.141),       // #1e1e24
        }
    }
}

pub static THEME: AppTheme = AppTheme::dark();

pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 12.0;
pub const SPACING_LG: f32 = 16.0;
pub const SPACING_XL: f32 = 24.0;

pub const RADIUS_SM: f32 = 4.0;
pub const RADIUS_MD: f32 = 8.0;
pub const RADIUS_LG: f32 = 12.0;

pub const SIDEBAR_WIDTH: f32 = 220.0;
pub const CARD_WIDTH: f32 = 200.0;
