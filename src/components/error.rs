use iced::{Background, Border, Color, Element, Length};
use iced::widget::{center, container, mouse_area, opaque, stack, Button, Column, Container, Text};

pub fn show<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    on_close: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = Container::new(
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .push(
                Container::new(
                    Column::new()
                        .width(Length::Fill)
                        .padding(20)
                        .spacing(10)
                        .push(Text::new("Dialog"))
                        .push(Button::new(Text::new("Close Dialog")).on_press(on_close))
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
