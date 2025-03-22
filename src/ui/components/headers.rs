use iced::{widget::{button, container, svg, text, Stack}, Color, Length, Padding, Theme};

use crate::{icons::close_icon, ui::Message};

pub fn close_n_text(function: Message, title: String) -> Stack<'static, Message, Theme>{
    let mut stack = Stack::new();
    stack = stack.push(
        container(
            text(title).size(32)
    ).padding(Padding::top(Padding::new(0.0), 30.0))
        .center_x(Length::Fill)
    );

    let handle = svg::Handle::from_memory(close_icon());
    let svg = svg(handle)
        .width(36)
        .height(44);

    stack = stack.push(
        container(
            button(svg)
            .on_press(function)
                .style(|_, _| button::Style{
                    background: None,
                    text_color: Color::WHITE,
                    ..Default::default()
                })
        )
    );
    return stack;
}