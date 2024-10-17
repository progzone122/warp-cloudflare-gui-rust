pub mod theme;
pub mod api;
pub mod pages;
pub mod embed;
pub mod components;

use iced::{widget, Element, Task, Theme};
use iced::application::View;
use iced::widget::{button, center, text, text_input, Column};
use iced::window::Settings;
use crate::api::Api;
use crate::components::error;
use crate::theme::{ACCENT_COLOR, PALETTE};

struct App {
    current_page: Page,
    theme: Theme,
    show_modal: bool,
    api: Api
}

pub enum Page {
    Home(pages::home::Home),
    Settings(pages::settings::Settings)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SwitchStatus(bool),
    OpenSettings,
    BackToHome,
    ShowModal,
    ErrorOkPressed
}

impl App {
    fn new() -> Self {
        let api: Api = Api::new();
        Self {
            current_page: Page::Home(pages::home::Home::new(api.is_connected())),
            theme: Theme::default(),
            show_modal: false,
            api
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
        match message {
            Message::ShowModal => {
                self.show_modal = true;
            }
            Message::ErrorOkPressed => {
                self.show_modal = false;
            }
            _ => {}
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let mut content = Column::new().spacing(20);

        match &self.current_page {
            Page::Home(home) => content = content.push(home.view()),
            Page::Settings(settings) => content = content.push(settings.view())
        }

        content = content.push(button(text("Show Modal")).on_press(Message::ShowModal));

        if self.show_modal {
            return error::show(content, Message::ErrorOkPressed);
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