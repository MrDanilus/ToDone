use levenshtein::levenshtein;

use crate::{
    ui::ToDo,
    core::update::save_all
};
use super::Message;

#[derive(Debug, Clone)]
pub enum TasksListMsg{
    SearchChange(String),
    CompleteTask(String),
    DeleteConfirm(String),
    DeleteTask(String)
}

pub fn handle(todo: &mut ToDo, message: TasksListMsg) {
    match message {
        // Обработка поиск
        TasksListMsg::SearchChange(req) => {
            todo.search_text = req;
            super::handle(todo, Message::LoadTasks);
            if todo.search_text.trim().len() > 1{
                let tasks = todo.tasks.clone();
                for task in tasks{
                    if levenshtein(&task.1.name, &todo.search_text.trim()) > 4{
                        todo.tasks.remove(&task.0);
                    }
                }
            }
        },
        // Выполнение задачи
        TasksListMsg::CompleteTask(id) => {
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
        // Удаление задачи
        TasksListMsg::DeleteConfirm(id) => todo.task_to_delete = id,
        TasksListMsg::DeleteTask(id) => {
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
    }
}