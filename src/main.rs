pub mod theme;
pub mod api;
pub mod pages;
pub mod embed;
pub mod components;

use iced::{Element, Task, Theme};
use iced::application::View;
use iced::widget::{Column};
use iced::window::Settings;
use crate::api::Api;
use crate::components::modal::Modal;
use crate::theme::{PALETTE};

struct App {
    current_page: Page,
    theme: Theme,
    api: Api,
    // Modal
    modal: Modal
}

pub enum Page {
    Home(pages::home::Home),
    Settings(pages::settings::Settings)
}

#[derive(Debug, Clone)]
enum Message {
    // Home
    SwitchStatus(bool),
    OpenSettings,
    // Modal
    ShowModal,
    ErrorOkPressed,
    SettingsMessage(pages::settings::Message)
}

impl App {
    fn new() -> Self {
        let api: Api = Api::new();
        Self {
            current_page: Page::Home(pages::home::Home::new(api.is_connected())),
            theme: Theme::default(),
            modal: Modal::default(),
            api
        }
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ShowModal => self.modal.show_default(),
            Message::ErrorOkPressed => self.modal.hide(),
            _ => match &mut self.current_page {
                Page::Home(home) => {
                    home.update(message.clone());
                    if let Message::OpenSettings = message {
                        self.current_page = Page::Settings(pages::settings::Settings::new());
                    }
                }
                Page::Settings(settings) => {
                    if let Message::SettingsMessage(ref settings_msg) = message {
                        settings.update(settings_msg.clone());
                        if let Message::SettingsMessage(pages::settings::Message::BackToHome) = message {
                            self.current_page = Page::Home(pages::home::Home::new(self.api.is_connected()));
                        }
                    }
                }
            }
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let mut content = match &self.current_page {
            Page::Home(home) => Column::new().spacing(20).push(home.view()),
            Page::Settings(settings) => Column::new().spacing(20).push(
                settings.view().map(Message::SettingsMessage)
            )
        };

        if self.modal.show {
            return self.modal.show_modal(content, Message::ErrorOkPressed);
        }

        content.into()
    }
    fn theme(&self) -> Theme {
        Theme::custom("Custom".to_string(), PALETTE)
    }
}
impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
fn main() -> iced::Result {
    iced::application("Warp GUI", App::update, App::view)
        .theme(App::theme)
        .window(Settings {
            size: iced::Size::new(300.0, 400.0),
            position: Default::default(),
            resizable: false,
            decorations: true,
            platform_specific: Default::default(),
            ..Default::default()
        })
        .run()
}