use iced::{Background, Color, Element};
use iced::widget::{center, container, mouse_area, opaque, stack, Button, Column, Text};

pub fn show<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    on_close: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let content = Column::new()
        .padding(20)
        .spacing(10)
        .push(Text::new("Dialog"))
        .push(Button::new(Text::new("Close Dialog")).on_press(on_close));

    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(Background::from(Color::from_rgba(0.0, 0.0, 0.0, 0.8))), // Using from_rgba for clarity
                    ..container::Style::default()
                }
            }))
        )
    ]
        .into()
}
