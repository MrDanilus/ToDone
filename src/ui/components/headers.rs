use iced::{widget::{button, container, text, Stack, Svg}, Color, Length, Padding, Theme};

use crate::core::update::Message;

pub fn button_n_text(svg: Svg<'static>, function: Message, title: String) -> Stack<'static, Message, Theme>{
    let mut stack = Stack::new();
    stack = stack.push(
        container(
            text(title).size(32)
    ).padding(Padding::top(Padding::new(0.0), 30.0))
        .center_x(Length::Fill)
    );

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