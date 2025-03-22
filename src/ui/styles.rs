use std::f32::INFINITY;
use std::sync::Arc;
use iced::border::Radius;
use iced::theme::{Custom, Palette};
use iced::{widget::button, Background, Border, Color, Theme};

use super::ToDo;

impl ToDo{
    pub fn theme(&self) -> Theme{
        Theme::Custom(
            Arc::new(Custom::new(String::from("Dark"), Palette{
                background: Color::parse("#111318").unwrap(),
                text: Color::parse("#f0f0f0").unwrap(),
                primary: Self::primary_color(),
                success: Self::primary_color(),
                danger: Self::primary_color()
            }))
        )
    }

    pub fn primary_color() -> Color{ Color::parse("#0f9aff").unwrap() }

    pub fn priority_button(todo: &ToDo, status: button::Status, priority: u8) -> button::Style{
        let priority_color = match priority{
            0 => Color::parse("#6B7280").unwrap(),
            1 => Color::parse("#FDBA74").unwrap(),
            2 => Color::parse("#F59E0B").unwrap(),
            3 => Color::parse("#EA580C").unwrap(),
            4 => Color::parse("#EF4444").unwrap(),
            _ => Color::from_rgb(0.2, 0.2, 0.2)
        };

        let mut style = button::Style{
            background: None,
            border: Border { 
                color: priority_color, 
                width: 1.0, 
                radius: Radius::new(INFINITY)
            },
            ..Default::default()
        };
        if todo.create.priority == priority{
            style.background = Some(Background::Color(priority_color));
        }
        match status {
            button::Status::Active => {},
            button::Status::Hovered => {
                if todo.create.priority != priority{
                    style.background = Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1)));
                }
            },
            button::Status::Pressed => {},
            button::Status::Disabled => {},
        }
        return style
    }
}