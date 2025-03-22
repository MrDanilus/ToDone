#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::futures::stream;
use iced::window::Settings;
use iced::{Size, Subscription};

mod ui;
use ui::{Message, ToDo};
mod core;

#[path="assets/icons/mod.rs"] mod icons;

pub fn main() -> iced::Result {
    iced::application("ToDo", core::update::func, ui::view::func)
        .centered()
        .theme(ToDo::theme)
        .window(Settings{
            size: Size::new(600.0, 700.0),
            resizable: false,
            ..Default::default()
        })
        .subscription(subscription)
        .run()
}

fn subscription(_: &ToDo) -> Subscription<Message> {
    Subscription::run_with_id((), stream::once(async { Message::LoadTasks }))
}