use iced::{Color};

pub mod button;
pub mod toggler;

/// Colors
pub const PRIMARY_COLOR: Color = Color::from_rgb(25.0 / 255.0, 25.0 / 255.0, 25.0 / 255.0);
pub const PRIMARY_COLOR_DARK: Color = Color::from_rgb(1.0 / 255.0, 1.0 / 255.0, 1.0 / 255.0);
pub const ACCENT_COLOR: Color = Color::from_rgb(255.0 / 255.0, 133.0 / 255.0, 57.0 / 255.0);
pub const TOGGLER_WARP_INACTIVE: Color = Color::from_rgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0);

/// Default settings
pub const DEFAULT_BUTTON_RADIUS: f32 = 5.0;
pub const DEFAULT_BORDER_WIDTH: f32 = 3.0;
pub const DEFAULT_BORDER_RADIUS: f32 = 10.0;

/// Warp toggler settings
pub const DEFAULT_TOGGLER_WARP_BACKGROUND_RADIUS: f32 = 5.0;

/// Theme color palette
pub const PALETTE: iced::theme::Palette = iced::theme::Palette {
    background: Color::from_rgb(0.0, 0.0, 0.0),
    text: Color::from_rgb(1.0, 1.0, 1.0),
    primary: Color::from_rgba(0.36862746, 0.4862745, 0.8862745, 1.0),
    success: Color::from_rgba(0.07058824, 0.4, 0.30980393, 1.0),
    danger: Color::from_rgba(0.7647059, 0.25882354, 0.24705882, 1.0),
};