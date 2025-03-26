use iced::{widget::{button, svg, Button}, Color, Length, Padding, Renderer, Theme};
use crate::{core::update::Message, ui::{styles::ToDoTheme, ToDo}};

pub fn hoverable(icon: &'static [u8], function: Message, size: (u16, u16)) -> Button<'static, Message, Theme, Renderer> {
    button(
        svg(svg::Handle::from_memory(icon))
            .width(Length::Fill).height(Length::Fill)
            .style(|theme, status| svg::Style { 
                color: Some(match status {
                    svg::Status::Idle => match ToDo::get_theme(theme){
                        ToDoTheme::Dark => Color::from_rgb(0.4, 0.4, 0.4),
                        ToDoTheme::Light => Color::from_rgb(0.6, 0.6, 0.6)
                    }
                    svg::Status::Hovered => match ToDo::get_theme(theme){
                        ToDoTheme::Dark => Color::from_rgb(0.8, 0.8, 0.8),
                        ToDoTheme::Light => Color::from_rgb(0.2, 0.2, 0.2)
                    }
                })
            })
    ).on_press(function)
    .style(|_, _| button::Style{
        background: None,
        ..Default::default()
    }).width(size.0).height(size.1)
    .padding(Padding::ZERO)
}