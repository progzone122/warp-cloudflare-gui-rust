use iced::widget::container::{self};
use iced::{Border, Color, Theme};


pub fn bottom_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(Color::from_rgb8(49, 52, 53))),
        ..container::Style::default()
    }
}