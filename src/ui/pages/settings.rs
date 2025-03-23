use iced::{
    widget::{
        column, container
    }, Element
};

use crate::{icons::arrow_back_icon, ui::{components::headers::button_n_text, Message, Page, ToDo}};

pub fn func(_: &ToDo) -> Element<Message> {
    container(
        column![
            button_n_text(arrow_back_icon(), Message::ChangePage(Page::TasksList), String::from("Настройки")),
        ]
    ).into()
}