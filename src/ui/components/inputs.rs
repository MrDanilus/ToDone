use iced::{
    border::Radius, 
    widget::{text_editor, text_input, TextEditor, TextInput}, 
    Background, Border, Color, Theme
};
use iced_core::text::highlighter::PlainText;

use crate::{core::update::Message, ui::{styles::ToDoTheme, ToDo}};

pub fn input(placeholder: &'static str, value: String, msg: impl Fn(String) -> Message + 'static) 
-> TextInput<'static, Message, Theme>{
    text_input(placeholder, &value)
        .on_input(msg)
        .style(|theme, status| text_input::Style { 
            background: Background::Color(Color::TRANSPARENT), 
            border: Border{
                color: match ToDo::get_theme(theme) {
                    ToDoTheme::Dark => match status{
                        text_input::Status::Hovered => 
                            Color::from_rgb(0.65, 0.65, 0.65),
                        text_input::Status::Focused => 
                            ToDo::primary_color(),
                        _ => Color::from_rgb(0.5, 0.5, 0.5),
                    }
                    ToDoTheme::Light => match status{
                        text_input::Status::Hovered => 
                            Color::from_rgb(0.55, 0.55, 0.55),
                        text_input::Status::Focused => 
                            ToDo::primary_color(),
                        _ => Color::from_rgb(0.7, 0.7, 0.7)
                    }
                },
                width: 1.0,
                radius: Radius::new(2)
            }, 
            icon: Color::TRANSPARENT, 
            placeholder: match ToDo::get_theme(theme) {
                ToDoTheme::Dark => Color::WHITE,
                ToDoTheme::Light => Color::BLACK
            }.scale_alpha(0.4), 
            value: match ToDo::get_theme(theme) {
                ToDoTheme::Dark => Color::WHITE,
                ToDoTheme::Light => Color::BLACK
            }, 
            selection: ToDo::primary_color()
        })
}

pub fn editor<'a>(
    placeholder: &'static str, value: &'a text_editor::Content, 
    action: impl Fn(text_editor::Action) -> Message + 'static) 
-> TextEditor<'a, PlainText, Message, Theme>{
    text_editor(value)
        .placeholder(placeholder)
        .on_action(action)
        .style(|theme, status| text_editor::Style { 
            background: Background::Color(Color::TRANSPARENT), 
            border: Border{
                color: match ToDo::get_theme(theme) {
                    ToDoTheme::Dark => match status{
                        text_editor::Status::Hovered => 
                            Color::from_rgb(0.65, 0.65, 0.65),
                            text_editor::Status::Focused => 
                            ToDo::primary_color(),
                        _ => Color::from_rgb(0.5, 0.5, 0.5),
                    }
                    ToDoTheme::Light => match status{
                        text_editor::Status::Hovered => 
                            Color::from_rgb(0.55, 0.55, 0.55),
                        text_editor::Status::Focused => 
                            ToDo::primary_color(),
                        _ => Color::from_rgb(0.7, 0.7, 0.7)
                    }
                },
                width: 1.0,
                radius: Radius::new(2)
            }, 
            icon: Color::TRANSPARENT, 
            placeholder: match ToDo::get_theme(theme) {
                ToDoTheme::Dark => Color::WHITE,
                ToDoTheme::Light => Color::BLACK
            }.scale_alpha(0.4), 
            value: match ToDo::get_theme(theme) {
                ToDoTheme::Dark => Color::WHITE,
                ToDoTheme::Light => Color::BLACK
            }, 
            selection: ToDo::primary_color()
        })
}