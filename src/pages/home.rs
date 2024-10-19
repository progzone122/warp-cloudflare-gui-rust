use std::sync::{OnceLock};
use iced::{Alignment, Color, ContentFit, Element, Font, Length, Padding, Theme};
use iced::widget::{button, column, container, image, row, text, toggler, Button, Container, Toggler};
use iced::widget::image::Handle;
use crate::api::Api;
use crate::embed::get_image;
use crate::Message;
use crate::Message::OpenSettings;
use crate::theme::{button::button_primary_style, toggler::toggler_warp_style, ACCENT_COLOR};
use crate::theme::button::button_icon_style;

static SETTINGS_IMAGE: OnceLock<Handle> = OnceLock::new();
static WATERMARK_IMAGE: OnceLock<Handle> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct Home {
    pub status: bool,
    api: Api,
}

impl Home {
    pub fn new(status: bool) -> Self {
        get_image(&SETTINGS_IMAGE, "settings.png");
        get_image(&WATERMARK_IMAGE, "watermark.png");
        Self {
            status, 
            api: Api::new()
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SwitchStatus(new_state) => {
                if new_state {
                    self.api.connect();
                } else {
                    self.api.disconnect();
                }
                self.status = self.api.is_connected();
                println!("Switching to {}", self.status);
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let settings_image_handle: &Handle = SETTINGS_IMAGE.get().expect("Failed to load settings image");
        let watermark_image_handle: &Handle = WATERMARK_IMAGE.get().expect("Failed to load watermark image");

        let button_settings: Button<Message> = button(
            image(settings_image_handle.clone())
                .width(Length::Fixed(20.0))
                .height(Length::Fill)
                .content_fit(ContentFit::Contain)
        )
            .style(button_icon_style)
            .on_press(OpenSettings);

        let bottom_container: Container<'_, Message, Theme> = container(row![
            image(watermark_image_handle.clone())
                .width(Length::Fixed(30.0))
                .height(Length::Fill)
                .content_fit(ContentFit::Contain),
            column![
                button_settings
            ]
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::End)
                .padding(Padding {
                    right: 10.0,
                    ..Padding::default()
                })

        ]
            .width(Length::Fill)
            .height(Length::Fixed(30.0))
            .align_y(Alignment::Center)
        )
            .padding(Padding {
                top: 5.0,
                bottom: 5.0,
                ..Padding::default()
            })
            .style(|theme_| {
                container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb8(49, 52, 53))),
                    ..container::Style::default()
                }
            });

        let toggler: Toggler<Message> = toggler(self.status)
            .on_toggle(|state| Message::SwitchStatus(state));

        column![
            column![
                text("WARP")
                    .size(50)
                    .font(Font::DEFAULT)
                    .color(ACCENT_COLOR),
                toggler.style(toggler_warp_style).size(80),
                text(if self.status { "Connected" } else { "Disconnected" }),
            ]
                .align_x(Alignment::Center)
                .padding(Padding::new(35.0))
                .width(Length::Fill)
                .height(Length::Fill),

            bottom_container
        ]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}