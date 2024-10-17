use iced::{Alignment, ContentFit, Element, Font, Length, Padding, Theme};
use iced::widget::{button, column, container, image, row, text, toggler, Button, Container, Image};
use iced::widget::image::Handle;
use crate::api::Api;
use crate::embed::load_image;
use crate::Message;
use crate::Message::OpenSettings;
use crate::theme::{button::button_primary_style, toggler::toggler_warp_style, ACCENT_COLOR};
use crate::theme::container::bottom_container_style;

#[derive(Clone, Debug)]
pub struct Home {
    pub status: bool,
    api: Api,
    //images
    settings_image: Handle,
    watermark_image: Handle
}

impl Home {
    pub fn new(status: bool) -> Self {
        Self { 
            status, 
            api: Api::new(),
            settings_image: load_image("settings.png").unwrap_or_else(|| {
                eprintln!("ERROR: Error loading settings image");
                "".into()
            }),
            watermark_image: load_image("watermark.png").unwrap_or_else(|| {
                eprintln!("ERROR: Error loading watermark image");
                "".into()
            }),

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
                let api_status: bool = self.api.is_connected();
                println!("Switching to {}", api_status);
                self.status = api_status;
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let button_settings: Button<Message> = button(
            image(self.settings_image.clone())
                .width(Length::Fixed(20.0))
                .height(Length::Fill)
                .content_fit(ContentFit::Contain)
        )
            .style(button_primary_style)
            .on_press(OpenSettings);

        let bottom_container: Container<'_, Message, Theme> = container(row![
            image(self.watermark_image.clone())
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
            .style(bottom_container_style);

        let toggler = toggler(self.status)
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