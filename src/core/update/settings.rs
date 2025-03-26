use crate::core::functions::settings::get::load;
use crate::ui::{styles::ToDoTheme, ToDo};
use super::Message;

#[derive(Debug, Clone)]
pub enum SettingsMsg{
    LoadSettings,
    ChangeDeleteConfirm,
    ChangeTheme(ToDoTheme)
}

pub fn handle(todo: &mut ToDo, message: SettingsMsg) {
    match message {
        // Загрузка настроек
        SettingsMsg::LoadSettings => match load() {
            Ok(res) => todo.settings = res,
            Err(err) => super::handle(todo, Message::Panic(err))
        },
        // Изменение подтверждения удаления
        SettingsMsg::ChangeDeleteConfirm => todo.settings.delete_confirm = !todo.settings.delete_confirm,
        // Смена темы программы
        SettingsMsg::ChangeTheme(theme) => todo.settings.theme = Some(theme)
    }
}