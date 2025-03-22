use crate::{core::tasks::create, ui::{Message, ToDo}};

use super::tasks::{get::get_all, save::rewrite_all};

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
            todo.create.error.clear();
            todo.create.success.clear();
            todo.page = page
        },

        // Список задач
        Message::SearchChange(req) => todo.search_text = req,
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
        Message::NameType(req) => todo.create.name = req,
        Message::DescriptionType(action) => todo.create.description.perform(action),
        Message::ChangePriority(req) => todo.create.priority = req,
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
                create::task(todo);
                todo.create.success = String::from("Успешно!");
                func(todo, Message::LoadTasks);
            }
        }
    }
}