use std::f32::INFINITY;
use std::sync::Arc;
use iced::border::Radius;
use iced::theme::{Custom, Palette};
use iced::{widget::button, Background, Border, Color, Theme};
use serde::{Deserialize, Serialize};

use super::ToDo;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub enum ToDoTheme{
    #[default]
    Dark,
    Light
}
impl std::fmt::Display for ToDoTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Dark => "Dark",
            Self::Light => "Light"
        })
    }
}

impl ToDo{
    pub fn theme(&self) -> Theme{
        Theme::Custom(
            if let Some(theme) = &self.settings.theme{
                match theme{
                    ToDoTheme::Dark => Arc::new(Custom::new(String::from("Dark"), Palette{
                        background: Color::parse("#111318").unwrap(),
                        text: Color::parse("#f0f0f0").unwrap(),
                        primary: Self::primary_color(),
                        success: Self::primary_color(),
                        danger: Self::primary_color()
                    })),
                    ToDoTheme::Light => Arc::new(Custom::new(String::from("Light"), Palette{
                        background: Color::parse("#f0f0f0").unwrap(),
                        text: Color::parse("#111318").unwrap(),
                        primary: Self::primary_color(),
                        success: Self::primary_color(),
                        danger: Self::primary_color()
                    }))
                }
            } else{
                Arc::new(Custom::new(String::from("Dark"), Palette{
                    background: Color::parse("#111318").unwrap(),
                    text: Color::parse("#f0f0f0").unwrap(),
                    primary: Self::primary_color(),
                    success: Self::primary_color(),
                    danger: Self::primary_color()
                }))
            }
        )
    }
    pub fn get_theme(theme: &Theme) -> ToDoTheme {
        match theme {
            Theme::Custom(theme) => {
                match theme.to_string().as_str() {
                    "Dark" => ToDoTheme::Dark,
                    "Light" => ToDoTheme::Light,
                    _ => ToDoTheme::Dark
                }
            },
            _ => ToDoTheme::Dark
        }
    }

    pub fn primary_color() -> Color{ Color::parse("#0f9aff").unwrap() }

    pub fn priority_button(
        current_theme: ToDoTheme, current_priority: u8, status: button::Status, priority: u8
    ) -> button::Style{
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
        if current_priority == priority{
            style.background = Some(Background::Color(priority_color));
        }
        match status {
            button::Status::Active => {},
            button::Status::Hovered => {
                if current_priority != priority{
                    style.background = Some(Background::Color(
                        match current_theme {
                            ToDoTheme::Dark => Color::from_rgb(0.1, 0.1, 0.1),
                            ToDoTheme::Light => Color::from_rgb(0.9, 0.9, 0.9)
                        }
                    ))
                }
            },
            button::Status::Pressed => {},
            button::Status::Disabled => {},
        }
        return style
    }
}