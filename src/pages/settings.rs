use std::clone::Clone;
use std::process::Command;
use std::sync::OnceLock;
use iced::{event, mouse, touch, Alignment, ContentFit, Element, Length, Padding};
use iced::advanced::graphics::text::cosmic_text::Align;
use iced::advanced::{Layout, Shell, Widget};
use iced::ContentFit::{Contain, Cover};
use iced::keyboard::key::Named::Link;
use iced::widget::{button, column, image, row, text, toggler, Button, Column, Row};
use iced::widget::image::Handle;
use iced::widget::shader::wgpu::naga::SwitchValue::Default;
use crate::embed::get_image;
use crate::Message;
use crate::theme::button::{button_icon_style, button_primary_style};

pub struct Settings {
    icons: LinksIcons
}

struct LinkIcon {
    icon: Handle,
    link: String
}
impl LinkIcon {
    pub fn new(icon: Handle, link: &str) -> Self {
        Self {
            icon,
            link: link.to_string(),
        }
    }
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
    pub fn get_icons(&self) -> [LinkIcon; 3] {
        [
            LinkIcon::new(Handle::from(self.github_icon.get().expect("GitHub icon not initialized")), "https://github.com/progzone122/warp-cloudflare-gui-rust"),
            LinkIcon::new(Handle::from(self.iced_icon.get().expect("Iced icon not initialized")), "https://iced.rs/"),
            LinkIcon::new(Handle::from(self.rust_icon.get().expect("Rust icon not initialized")), "https://www.rust-lang.org/")
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
            Message::SettingsLinkIconPressed(url) => {
                if let Err(e) = Command::new("xdg-open").arg(&url).spawn() {
                    eprintln!("Error: Failed to open link ({})", e);
                }
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let icon_handles: [LinkIcon; 3] = self.icons.get_icons();

        let mut icons_link_row: Row<Message> = Row::new()
            .height(Length::Fill)
            .spacing(10)
            .align_y(Alignment::End);

        for icon_handle in icon_handles {

            icons_link_row = icons_link_row.push(
                button(
                    image(icon_handle.icon)
                )
                    .width(Length::Fixed(50.0))
                    .height(Length::Fixed(50.0))
                    .padding(10)
                    .style(button_icon_style)
                    .on_press(Message::SettingsLinkIconPressed(icon_handle.link))
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
