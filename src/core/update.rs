use std::sync::Arc;
use iced::widget::text_editor::{Action, Edit};
use levenshtein::levenshtein;

use crate::{core::tasks::create, ui::{Message, Page, ToDo}};
use super::tasks::{get::get_all, save::{rewrite_all, save_task}};

pub fn func(todo: &mut ToDo, message: Message) {
    match message {
        // Система
        Message::LoadTasks => {
            match get_all(){
                Ok(res) => todo.tasks = res,
                Err(err) => { Message::Panic(err); }
            };
            // Сортировка задач
            todo.tasks.sort_by(|task1, task2| task2.priority.cmp(&task1.priority));
            todo.tasks.sort_by_key(|task| task.completed.clone());
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
                    todo.edit.id = id;
                    let index = todo.tasks.iter().position(|x| x.id == todo.edit.id);
                    match index {
                        Some(index) => {
                            let task = todo.tasks.iter().nth(index).unwrap().clone();
                            todo.edit.name = task.name;
                            todo.edit.description.perform(Action::Edit(
                                Edit::Paste(Arc::new(task.description)))
                            );
                            todo.edit.priority = task.priority;
                        },
                        None => {}
                    }
                    todo.edit.error.clear();
                    todo.edit.success.clear();
                }
            }
            todo.page = page
        },

        // Список задач
        Message::SearchChange(req) => {
            todo.search_text = req;
            func(todo, Message::LoadTasks);
            if todo.search_text.trim().len() > 1{
                let tasks = todo.tasks.clone();
                for i in 0..tasks.len(){
                    let task = tasks.get(i).unwrap();
                    if levenshtein(&task.name, &todo.search_text.trim()) > 4{
                        let index = todo.tasks.iter().position(|x| x.id == task.id).unwrap();
                        todo.tasks.remove(index);
                    }
                }
            }
        },
        Message::CompleteTask(id) => {
            let index = todo.tasks.iter().position(|x| x.id == id);
            match index {
                Some(index) => {
                    let task = todo.tasks.iter().nth(index).unwrap().clone();
                    todo.tasks.iter_mut().nth(index).unwrap().completed = !task.completed;
                    if let Err(err) = rewrite_all(todo.tasks.clone()){
                        todo.panic = err
                    }
                    // Сортировка задач
                    todo.tasks.sort_by(|task1, task2| task2.priority.cmp(&task1.priority));
                    todo.tasks.sort_by_key(|task| task.completed.clone());
                },
                None => {}
            }
        },
        Message::DeleteTask(id) => {
            let index = todo.tasks.iter().position(|x| x.id == id);
            match index {
                Some(index) => {
                    todo.tasks.remove(index);
                    if let Err(err) = rewrite_all(todo.tasks.clone()){
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
            if name.len() < 2 || name.len() > 128{
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
            if name.len() < 2 || name.len() > 128{
                todo.edit.error = String::from("Длина названия должна быть от 2 до 128 символов");
            } else if description.trim().len() > 2048{
                todo.edit.error = String::from("Длина описания не может быть больше 2048 символов");
            }
            else{
                let index = todo.tasks.iter().position(|x| x.id == todo.edit.id);
                match index {
                    Some(index) => {
                        let mut task = todo.tasks.iter().nth(index).unwrap().clone();
                        task.name = todo.edit.name.clone();
                        task.description = todo.edit.description.text();
                        task.priority = todo.edit.priority;
                        // Сохранение задачи
                        if let Err(err) = save_task(task){
                            todo.edit.error = err;
                        } else{
                            func(todo, Message::LoadTasks);
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