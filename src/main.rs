#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::futures::stream;
use iced::window::Settings;
use iced::{Size, Subscription};

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
            size: Size::new(600.0, 700.0),
            resizable: false,
            ..Default::default()
        })
        // Запуск функции для загрузки данных
        .subscription(subscription)
        .run()
}

// Функция, выполняемая при запуске программы
fn subscription(_: &ToDo) -> Subscription<Message> {
    Subscription::run_with_id((), stream::iter(vec![ 
        Message::LoadTasks, Message::Settings(SettingsMsg::LoadSettings)
    ]))
}