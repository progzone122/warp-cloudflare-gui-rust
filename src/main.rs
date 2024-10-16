pub mod theme;
pub mod api;
pub mod pages;
pub mod embed;

use iced::{Element, Task, Theme};
use iced::application::View;
use iced::window::Settings;
use crate::api::Api;
use crate::theme::{ACCENT_COLOR, PALETTE};
use embed::Images;

struct App {
    current_page: Page,
    theme: Theme
}

pub enum Page {
    Home(pages::home::Home),
    Settings(pages::settings::Settings)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SwitchStatus(bool),
    OpenSettings,
    BackToHome
}

impl App {
    fn new() -> Self {
        let api: Api = Api::new();
        Self {
            current_page: Page::Home(pages::home::Home::new(api.is_connected())),
            theme: Theme::default()
        }
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        let api: Api = Api::new();
        match &mut self.current_page {
            Page::Home(home) => {
                home.update(message);
                if let Message::OpenSettings = message {
                    self.current_page = Page::Settings(pages::settings::Settings::new());
                }
            }
            Page::Settings(settings) => {
                settings.update(message);
                if let Message::BackToHome = message {
                    self.current_page = Page::Home(pages::home::Home::new(api.is_connected()));
                }
            }
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        match &self.current_page {
            Page::Home(home) => home.view(),
            Page::Settings(settings) => settings.view()
        }
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