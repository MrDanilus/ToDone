#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::futures::stream;
use iced::window::{self, Settings};
use iced::{event, Size, Subscription};

mod ui;
use ui::ToDo;
mod core;
use core::update::{
    settings::SettingsMsg,
    Message
};

#[path="assets/icons/mod.rs"] mod icons;

pub fn main() -> iced::Result {
    iced::application("ToDone", core::update::handle, ui::view::func)
        .centered()
        .theme(ToDo::theme)
        .window(Settings{
            size: Size::new(700.0, 750.0),
            resizable: true,
            ..Default::default()
        })
        // Запуск функции для загрузки данных
        .subscription(subscription)
        .run()
}

fn subscription(_: &ToDo) -> Subscription<Message> {
    Subscription::batch(vec![ 
        // Функции, выполняемая при запуске программы
        Subscription::run(|| stream::iter(vec![
            Message::LoadTasks, Message::Settings(SettingsMsg::LoadSettings)
        ])),
        // Установка слушателя событий
        event::listen_with(|event, _status, _window| {
            match event {
                iced::Event::Window(window::Event::Resized(size)) => {
                    Some(Message::WindowResized(size.width as u32, size.height as u32))
                }
                _ => None
            }
        })
    ])
}