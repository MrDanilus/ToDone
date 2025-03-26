use iced::{
    border::Radius, 
    widget::{button, Button}, 
    Background, Border, Color, Renderer, Theme
};
use crate::{
    core::update::Message, 
    ui::ToDo
};

pub fn hoverable(text: &'static str, function: Message) -> Button<'static, Message, Theme, Renderer> {
    button(text)
        .style(|_, status| button::Style { 
            text_color: Color::WHITE,
            background: Some(Background::Color(
                match status{
                    button::Status::Hovered => ToDo::primary_color(),
                    _ => ToDo::primary_color().scale_alpha(0.85)
                })),
            border: Border{
                radius: Radius::new(5),
                ..Default::default()
            },
            ..Default::default() 
        }).on_press(function)
}