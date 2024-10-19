use std::sync::OnceLock;
use iced::{Background, Border, Color, ContentFit, Element, Length};
use iced::widget::image::Handle;
use iced::widget::{center, container, image, mouse_area, opaque, stack, Button, Column, Container, Row, Text};
use crate::embed::get_image;
use crate::theme::button::button_primary_style;

static WARNING_IMAGE: OnceLock<Handle> = OnceLock::new();

pub fn show<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    on_close: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    get_image(&WARNING_IMAGE, "warning.png");

    let warning_image_handle: &Handle = WARNING_IMAGE.get().expect("Failed to load warning image");

    let content = Container::new(
        Column::new()
            .width(Length::Fill)
            .padding(10)
            .push(
                Container::new(
                    Column::new()
                        .width(Length::Fill)
                        .padding(15)
                        .spacing(10)
                        .push(
                            Row::new()
                                .width(Length::Fill)
                                .push(
                                    image(warning_image_handle.clone())
                                        .width(Length::Fixed(35.0))
                                        .height(Length::Fixed(40.0))
                                        .content_fit(ContentFit::Contain)
                                )
                                .push(
                                    Column::new()
                                        .push(Text::new("ERROR"))
                                        .push(Text::new("Error message"))
                                        .spacing(5)
                                )
                                .spacing(13)
                        )
                        .push(
                            Button::new(Text::new("OK"))
                                .style(button_primary_style)
                                .on_press(on_close)
                        )
                )
                    .style(|_theme| container::Style {
                        background: Some(Background::Color(Color::from_rgb8(49, 52, 53))),
                        border: Border::default().rounded(10.0),
                        ..Default::default()
                    })
            )
    );

    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(Background::from(Color::from_rgba(0.0, 0.0, 0.0, 0.8))),
                    ..container::Style::default()
                }
            }))
        )
    ]
        .into()
}
