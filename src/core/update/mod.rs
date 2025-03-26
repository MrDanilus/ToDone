use std::sync::Arc;
use iced::widget::text_editor::{Action, Edit, Motion};

pub mod tasks_list;
use tasks_list::TasksListMsg;
pub mod tasks;
use tasks::TasksMsg;
pub mod settings;
use settings::SettingsMsg;
pub mod menu;
use menu::MenuMsg;

use crate::ui::{Page, ToDo};
use super::{functions::settings::save::save, functions::tasks::{get::get_all, save::save_all}};

#[derive(Debug, Clone)]
pub enum Message {
    // Система
    LoadTasks,
    // Страницы
    Panic(String),
    ChangePage(Page),

    // Действия
    //// Список задач
    TasksList(TasksListMsg),
    //// Задачи
    Tasks(TasksMsg),
    //// Настройки
    Settings(SettingsMsg),
    //// Меню
    Menu(MenuMsg)
}

pub fn handle(todo: &mut ToDo, message: Message) {
    match message {
        // Система
        //// Загрузка задач из файла
        Message::LoadTasks => {
            match get_all(){
                Ok(res) => todo.tasks = res,
                Err(err) => { Message::Panic(err); }
            };
        },

        // Страницы
        //// Обработка ошибок
        Message::Panic(err) => todo.panic = err,
        //// Смена страницы
        Message::ChangePage(page) => {
            match page.clone() {
                Page::TasksList => {
                    // Сохранение настроек
                    match save(todo.settings.clone()) {
                        Ok(_) => {},
                        Err(err) => handle(todo, Message::Panic(err))
                    };
                    // Очистка переменных
                    todo.create.name.clear();
                    todo.create.priority = 0;
                    todo.edit.name.clear();
                    todo.edit.priority = 0;
                    for _ in 0..3000{
                        if todo.create.description.cursor_position().1 == 0{
                            todo.create.description.perform(Action::Move(Motion::Up));
                            todo.create.description.perform(Action::Move(Motion::End));
                        }
                        todo.create.description.perform(Action::Edit(Edit::Backspace));
                        if todo.edit.description.cursor_position().1 == 0{
                            todo.edit.description.perform(Action::Move(Motion::Up));
                            todo.edit.description.perform(Action::Move(Motion::End));
                        }
                        todo.edit.description.perform(Action::Edit(Edit::Backspace));
                    }
                },
                Page::CreateTask => {
                    todo.create.error.clear();
                    todo.create.success.clear();
                },
                Page::EditTask(id) => {
                    todo.edit.id = id.clone();
                    let task = todo.tasks.get(&id);
                    match task {
                        Some(task) => {
                            todo.edit.name = task.name.clone();
                            todo.edit.description.perform(Action::Edit(
                                Edit::Paste(Arc::new(task.description.clone())))
                            );
                            todo.edit.priority = task.priority;
                        },
                        None => {}
                    }
                    todo.edit.error.clear();
                    todo.edit.success.clear();
                },
                Page::Menu => settings::handle(todo, SettingsMsg::LoadSettings),
            }
            todo.search_text.clear();
            todo.page = page
        },

        // Список задач
        Message::TasksList(msg) => tasks_list::handle(todo, msg),
        // Действия с задачами
        Message::Tasks(msg) => tasks::handle(todo, msg),
        // Настройки
        Message::Settings(msg) => settings::handle(todo, msg),
        // Меню
        Message::Menu(msg) => menu::handle(todo, msg)
    }
}