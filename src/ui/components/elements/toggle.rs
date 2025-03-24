use std::f32::INFINITY;
use iced::{
    alignment::Horizontal, border::Radius, 
    widget::{button, container, Space}, 
    Background, Border, Color, Element, Padding, Renderer, Theme
};

use crate::ui::{styles::ToDoTheme, Message, ToDo};

pub fn func(status: bool, msg: Message) -> Element<'static, Message, Theme, Renderer>{
    container(
        button(
            container(
                container(
                    Space::new(14, 18)
                    ).style(move |theme|{ 
                        container::Style { 
                            background: Some(Background::Color(
                                if !status {match ToDo::get_theme(theme) {
                                    ToDoTheme::Dark => Color::WHITE,
                                    ToDoTheme::Light => Color::BLACK
                                }} else{ Color::WHITE }
                            )), 
                            border: Border{
                                color: match ToDo::get_theme(theme) {
                                    ToDoTheme::Dark => Color::WHITE,
                                    ToDoTheme::Light => Color::BLACK
                                },
                                width: 0.0,
                                radius: Radius::new(INFINITY)
                            },
                            ..Default::default()
                        }
                        }
                    )
            )
            .align_x(
                if status { Horizontal::Right } 
                else{ Horizontal::Left })
                .padding(Padding::new(6.0))
            .style(move |theme| container::Style { 
                background: if status{
                    Some(Background::Color(Color::from_rgb(0.2, 0.6, 0.2)))
                } else{
                    Some(Background::Color(Color::TRANSPARENT))
                }, 
                border: Border{
                    color: match ToDo::get_theme(theme) {
                        ToDoTheme::Dark => Color::WHITE,
                        ToDoTheme::Light => Color::BLACK
                    },
                    width: 2.0,
                    radius: Radius::new(INFINITY)
                },
                ..Default::default()
            }).width(48).height(26)
        ).on_press(msg).style(|_, _| button::Style::default())
    ).into()
}