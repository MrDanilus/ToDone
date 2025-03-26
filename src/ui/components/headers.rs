use iced::{
    widget::{container, text, Stack},
    Length, Padding, Theme
};

use crate::core::update::Message;
use super::buttons;

pub fn button_n_text(icon: &'static [u8], msg: Message, title: String) -> Stack<'static, Message, Theme>{
    let mut stack = Stack::new();
    stack = stack.push(
        container(
            text(title).size(32)
    ).padding(Padding::top(Padding::new(0.0), 30.0))
        .center_x(Length::Fill)
    );

    stack = stack.push(
        container(
            buttons::icon::hoverable(
                icon, msg, (32, 40)
        )).padding(Padding::from([10.0, 12.0]))
    );
    return stack;
}