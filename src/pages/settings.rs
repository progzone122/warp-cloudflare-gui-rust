use iced::Element;
use iced::widget::{button, column, image, text, toggler};
use crate::Message;
use crate::theme::button::button_primary_style;

pub struct Settings;

impl Settings {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, message: Message) {
        match message {
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text("Settings")
                .size(30),
            button("Back")
                .style(button_primary_style)
                .on_press(Message::BackToHome)
        ].into()
    }
}
