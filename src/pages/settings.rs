use std::clone::Clone;
use std::sync::OnceLock;
use iced::{event, mouse, touch, Alignment, ContentFit, Element, Length, Padding};
use iced::advanced::graphics::text::cosmic_text::Align;
use iced::advanced::{Layout, Shell, Widget};
use iced::ContentFit::{Contain, Cover};
use iced::widget::{button, column, image, row, text, toggler, Button, Column, Row};
use iced::widget::image::Handle;
use iced::widget::shader::wgpu::naga::SwitchValue::Default;
use crate::embed::get_image;
use crate::Message;
use crate::theme::button::{button_icon_style, button_primary_style};

pub struct Settings {
    icons: LinksIcons
}

struct LinksIcons {
    github_icon: OnceLock<Handle>,
    iced_icon: OnceLock<Handle>,
    rust_icon: OnceLock<Handle>,
}
impl LinksIcons {
    pub fn new() -> Self {
        Self {
            github_icon: OnceLock::new(),
            iced_icon: OnceLock::new(),
            rust_icon: OnceLock::new(),
        }
    }
    pub fn init(&self) {
        get_image(&self.github_icon, "github-icon.png");
        get_image(&self.iced_icon, "iced-icon.png");
        get_image(&self.rust_icon, "rust-icon.png");
    }
    pub fn get_icons(&self) -> [&Handle; 3] {
        [
            self.github_icon.get().expect("GitHub icon not initialized"),
            self.iced_icon.get().expect("Iced icon not initialized"),
            self.rust_icon.get().expect("Rust icon not initialized"),
        ]
    }
}

impl Settings {
    pub fn new() -> Self {
        let icons: LinksIcons = LinksIcons::new();
        icons.init();

        Self {
            icons
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let icon_handles: [&Handle; 3] = self.icons.get_icons();

        let mut icons_link_row: Row<Message> = Row::new()
            .height(Length::Fill)
            .spacing(10)
            .align_y(Alignment::End);

        for icon_handle in icon_handles {

            icons_link_row = icons_link_row.push(
                button(
                    image(icon_handle.clone())
                )
                    .width(Length::Fixed(50.0))
                    .height(Length::Fixed(50.0))
                    .padding(10)
                    .style(button_icon_style)
                    .on_press(Message::ImageClicked)
            )
        }

        let icons_link_column: Column<Message> = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .push(icons_link_row);

        column![
            button("Back")
                .style(button_primary_style)
                .on_press(Message::BackToHome),

            column![
                text(format!("Version: {}", env!("CARGO_PKG_VERSION")))
                .size(18),
            ]
                .spacing(10),

            icons_link_column
        ]
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .spacing(30)
            .into()
    }
}
