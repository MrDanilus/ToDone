use std::sync::Arc;
use iced::widget::text_editor::{Action, Edit, Motion};
use levenshtein::levenshtein;

use crate::{core::tasks::create, ui::{Message, Page, ToDo}};
use super::{settings::{get::load, save::save}, tasks::{get::get_all, save::save_all}};

pub fn func(todo: &mut ToDo, message: Message) {
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
                        Err(err) => func(todo, Message::Panic(err))
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
                Page::Settings => func(todo, Message::LoadSettings),
            }
            todo.search_text.clear();
            todo.page = page
        },

        // Список задач
        //// Обработка поиска
        Message::SearchChange(req) => {
            todo.search_text = req;
            func(todo, Message::LoadTasks);
            if todo.search_text.trim().len() > 1{
                let tasks = todo.tasks.clone();
                for task in tasks{
                    if levenshtein(&task.1.name, &todo.search_text.trim()) > 4{
                        todo.tasks.remove(&task.0);
                    }
                }
            }
        },
        //// Выполнение задачи
        Message::CompleteTask(id) => {
            let task = todo.tasks.get_mut(&id);
            match task {
                Some(task) => {
                    task.completed = !task.completed;
                    if let Err(err) = save_all(todo.tasks.clone()){
                        todo.panic = err
                    }
                },
                None => {}
            }
        },
        //// Удаление задачи
        Message::DeleteConfirm(id) => todo.task_to_delete = id,
        Message::DeleteTask(id) => {
            match todo.tasks.get(&id) {
                Some(_) => {
                    todo.tasks.remove(&id);
                    if let Err(err) = save_all(todo.tasks.clone()){
                        todo.panic = err
                    }
                },
                None => {}
            }
        }

        // Новая задача
        //// Ввод названия в переменную
        Message::NameCreateType(req) => todo.create.name = req,
        //// Ввод описания в переменную
        Message::DescriptionCreateType(action) => {
            match action {
                Action::Edit(Edit::Enter) => {},
                _ => todo.create.description.perform(action)
            }
        },
        //// Смена приоритета в переменной
        Message::PriorityCreateChange(req) => todo.create.priority = req,
        //// Создание задачи
        Message::CreateTask => {
            todo.create.error.clear();
            todo.create.success.clear();
            let name = todo.create.name.clone();
            let description = todo.create.description.text().clone();
            if name.trim().len() < 2 || name.trim().len() > 128{
                todo.create.error = String::from("Длина названия должна быть от 2 до 128 символов");
            } else if description.trim().len() > 2048{
                todo.create.error = String::from("Длина описания не может быть больше 2048 символов");
            }
            else{
                if let Some(msg) = create::task(todo){
                    func(todo, msg);
                }
                func(todo, Message::LoadTasks);
                func(todo, Message::ChangePage(Page::TasksList));
            }
        },

        // Редактирование задачи
        //// Ввод названия в переменную
        Message::NameEditType(req) => todo.edit.name = req,
        //// Ввод описания в переменную
        Message::DescriptionEditType(action) => {
            match action {
                Action::Edit(Edit::Enter) => {},
                _ => todo.edit.description.perform(action)
            }
        },
        //// Смена приоритета в переменной
        Message::PriorityEditChange(req) => todo.edit.priority = req,
        ////Изменение задачи
        Message::EditTask => {
            todo.edit.error.clear();
            todo.edit.success.clear();
            let name = todo.edit.name.clone();
            let description = todo.edit.description.text().clone();
            if name.trim().len() < 2 || name.trim().len() > 128{
                todo.edit.error = String::from("Длина названия должна быть от 2 до 128 символов");
            } else if description.trim().len() > 2048{
                todo.edit.error = String::from("Длина описания не может быть больше 2048 символов");
            }
            else{
                let task = todo.tasks.get_mut(&todo.edit.id);
                match task {
                    Some(task) => {
                        task.name = todo.edit.name.clone().trim().to_string();
                        task.description = todo.edit.description.text().trim().to_string();
                        task.priority = todo.edit.priority;
                        // Сохранение задачи
                        if let Err(err) = save_all(todo.tasks.clone()){
                            todo.edit.error = err;
                        } else{
                            func(todo, Message::ChangePage(Page::TasksList));
                        }
                    },
                    None => {
                        todo.edit.error = format!("Задача с ID: {} не найдена", todo.edit.id);
                    }
                }
            }
        },

        // Настройки
        //// Загрузка настроек
        Message::LoadSettings => match load() {
            Ok(res) => todo.settings = res,
            Err(err) => func(todo, Message::Panic(err))
        },
        ///// Изменение подтверждения удаления
        Message::ChangeDeleteConfirm => todo.settings.delete_confirm = !todo.settings.delete_confirm,
        //// Смена темы программы
        Message::ChangeTheme(theme) => todo.settings.theme = Some(theme)
    }
}