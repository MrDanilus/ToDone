use std::sync::Arc;
use iced::widget::text_editor::{Action, Edit};
use levenshtein::levenshtein;

use crate::{core::tasks::create, ui::{Message, Page, ToDo}};
use super::tasks::{get::get_all, save::save_all};

pub fn func(todo: &mut ToDo, message: Message) {
    match message {
        // Система
        Message::LoadTasks => {
            match get_all(){
                Ok(res) => todo.tasks = res,
                Err(err) => { Message::Panic(err); }
            };
        },

        // Страницы
        Message::Panic(err) => todo.panic = err,
        Message::ChangePage(page) => {
            match page.clone() {
                Page::TasksList => {
                    todo.create.name.clear();
                    todo.create.priority = 0;
                    todo.edit.name.clear();
                    todo.edit.priority = 0;
                    for _ in 0..3000{
                        todo.create.description.perform(Action::Edit(Edit::Backspace));
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
                Page::Settings => {}
            }
            todo.search_text.clear();
            todo.page = page
        },

        // Список задач
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
        Message::NameCreateType(req) => todo.create.name = req,
        Message::DescriptionCreateType(action) => {
            match action {
                Action::Edit(Edit::Enter) => {},
                _ => todo.create.description.perform(action)
            }
        },
        Message::PriorityCreateChange(req) => todo.create.priority = req,
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
        Message::NameEditType(req) => todo.edit.name = req,
        Message::DescriptionEditType(action) => {
            match action {
                Action::Edit(Edit::Enter) => {},
                _ => todo.edit.description.perform(action)
            }
        },
        Message::PriorityEditChange(req) => todo.edit.priority = req,
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
        }
    }
}