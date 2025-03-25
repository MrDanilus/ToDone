use iced::{
    widget::{
        column, container, text
    }, Color, Element, Length, Padding
};

use crate::{core::update::Message, ui::ToDo};

pub fn func(todo: &ToDo) -> Element<Message> {
    container(
        column![
            container(
                text("ОШИБКА!")
                    .size(32)
            ).padding(Padding::top(Padding::new(0.0), 30.0))
                .center_x(Length::Fill),
            container(
                text(format!("Во время работы программы возникла фатальная ошибка.\nОшибка: {}", todo.panic))
                    .color(Color::from_rgb(1.0, 0.0, 0.0))
            ).center(Length::Fill)
            .padding(Padding::bottom(Padding::default(), 50.0)),
        ]
    ).into()
}