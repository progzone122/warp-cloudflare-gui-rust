use std::clone::Clone;
use std::default::Default;
use std::process::Command;
use std::sync::OnceLock;
use iced::{Alignment, Element, Length};
use iced::advanced::{ Widget};
use iced::widget::{button, column, horizontal_rule, image, row, text, Column, Row};
use iced::widget::image::Handle;
use crate::api::Api;
use crate::components::modal::{Modal, TypeModal};
use crate::embed::get_image;
use crate::theme::button::{button_icon_style, button_primary_style};


#[derive(Clone, Debug)]
pub enum Message {
    BackToHome,
    SettingsLinkIconPressed(String),
    AccountRegister(bool), // true - register, false - delete
    ErrorOkPressed
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
    cloudflare_icon: OnceLock<Handle>,
}
impl LinksIcons {
    pub fn new() -> Self {
        Self {
            github_icon: OnceLock::new(),
            iced_icon: OnceLock::new(),
            rust_icon: OnceLock::new(),
            cloudflare_icon: OnceLock::new()
        }
    }
    pub fn init(&self) {
        get_image(&self.github_icon, "github-icon.png");
        get_image(&self.iced_icon, "iced-icon.png");
        get_image(&self.rust_icon, "rust-icon.png");
        get_image(&self.cloudflare_icon, "cloudflare-icon.png");
    }
    pub fn get_icons(&self) -> [LinkIcon; 4] {
        [
            LinkIcon::new(Handle::from(self.github_icon.get().expect("GitHub icon not initialized")), "https://github.com/progzone122/warp-cloudflare-gui-rust"),
            LinkIcon::new(Handle::from(self.iced_icon.get().expect("Iced icon not initialized")), "https://iced.rs/"),
            LinkIcon::new(Handle::from(self.rust_icon.get().expect("Rust icon not initialized")), "https://www.rust-lang.org/"),
            LinkIcon::new(Handle::from(self.cloudflare_icon.get().expect("Cloudflare icon not initialized")), "https://one.one.one.one/")
        ]
    }
}

pub struct Settings {
    icons: LinksIcons,
    api: Api,
    modal: Modal
}

impl Settings {
    pub fn new() -> Self {
        let icons: LinksIcons = LinksIcons::new();
        icons.init();

        let api: Api = Api::new();

        Self {
            icons,
            api,
            modal: Modal::default(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SettingsLinkIconPressed(url) => {
                if let Err(e) = Command::new("xdg-open").arg(&url).spawn() {
                    eprintln!("Error: Failed to open link ({})", e);
                }
            }
            Message::AccountRegister(action) => {
                if action {
                    match self.api.register_account() {
                        Ok(res) => {}
                        Err(e) => {
                            self.modal.show(TypeModal::Error, &format!("Failed to register account ({})", e));
                            eprintln!("Error: Failed to register account ({})", e);
                        }
                    }
                } else {
                    match self.api.delete_account() {
                        Ok(res) => {}
                        Err(e) => {
                            self.modal.show(TypeModal::Error, &format!("Failed to register account ({})", e));
                            eprintln!("Error: Failed to delete account ({})", e);
                        }
                    }
                }
            }
            Message::ErrorOkPressed => self.modal.hide(),
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {

        let icon_handles: [LinkIcon; 4] = self.icons.get_icons();

        let mut icons_link_row: Row<Message> = Row::new()
            .height(Length::Fill)
            .spacing(7)
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

        let mut content: Element<Message> = column![
            button("Back")
                .style(button_primary_style)
                .on_press(Message::BackToHome),

            column![
                text(format!("Version: {}", env!("CARGO_PKG_VERSION")))
                    .size(18),
                horizontal_rule(0),
                row![
                    text("Account:")
                        .size(18),
                    column![
                        row![
                            button("Register")
                                .style(button_primary_style)
                                .on_press(Message::AccountRegister(true)),
                            button("Delete")
                                .style(button_primary_style)
                                .on_press(Message::AccountRegister(false))
                        ]
                            .spacing(10)
                    ]
                        .width(Length::Fill)
                        .align_x(Alignment::End)
                ]
                    .align_y(Alignment::Center),
            ]
                .spacing(10),
            icons_link_column
        ]
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .spacing(30)
            .into();

        if self.modal.show {
            self.modal.show_modal(content, Message::ErrorOkPressed)
        } else {
            content
        }
    }
}
