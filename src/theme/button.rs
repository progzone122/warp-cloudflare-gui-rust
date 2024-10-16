use iced::widget::button::{self, Status};
use iced::{Border, Color, Theme};

use super::{
    // Colors
    PRIMARY_COLOR,
    PRIMARY_COLOR_DARK,
    // Other settings
    DEFAULT_BUTTON_RADIUS,
    DEFAULT_BORDER_WIDTH,
    DEFAULT_BORDER_RADIUS,
    ACCENT_COLOR
};

pub fn button_primary_style(_theme: &Theme, status: Status) -> button::Style {
    match status {
        Status::Active => button::Style {
            background: Some(iced::Background::Color(PRIMARY_COLOR)),
            text_color: Color::WHITE,
            border: Border::default().rounded(DEFAULT_BUTTON_RADIUS),
            ..button::Style::default()
        },
        _ => button::Style {
            background: Some(iced::Background::Color(ACCENT_COLOR)),
            text_color: Color::WHITE,
            border: Border::default().rounded(DEFAULT_BUTTON_RADIUS),
            ..button::Style::default()
        }
    }
}
