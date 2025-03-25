use iced::widget::text_editor::{self, Action, Edit};

use crate::{
    core::{
        functions::tasks::create,
        update::save_all
    }, 
    ui::{Page, ToDo}
};
use super::Message;

#[derive(Debug, Clone)]
pub enum TasksMsg{
    // Новая задача
    NameCreateType(String),
    DescriptionCreateType(text_editor::Action),
    PriorityCreateChange(u8),
    CreateTask,
    // Редактирование задачи
    NameEditType(String),
    DescriptionEditType(text_editor::Action),
    PriorityEditChange(u8),
    EditTask,
}

pub fn handle(todo: &mut ToDo, message: TasksMsg) {
    match message {
        // Новая задача
        //// Ввод названия в переменную
        TasksMsg::NameCreateType(req) => todo.create.name = req,
        //// Ввод описания в переменную
        TasksMsg::DescriptionCreateType(action) => {
            match action {
                Action::Edit(Edit::Enter) => {},
                _ => todo.create.description.perform(action)
            }
        },
        //// Смена приоритета в переменной
        TasksMsg::PriorityCreateChange(req) => todo.create.priority = req,
        //// Создание задачи
        TasksMsg::CreateTask => {
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
                    super::handle(todo, msg);
                }
                super::handle(todo, Message::LoadTasks);
                super::handle(todo, Message::ChangePage(Page::TasksList));
            }
        },

        // Редактирование задачи
        //// Ввод названия в переменную
        TasksMsg::NameEditType(req) => todo.edit.name = req,
        //// Ввод описания в переменную
        TasksMsg::DescriptionEditType(action) => {
            match action {
                Action::Edit(Edit::Enter) => {},
                _ => todo.edit.description.perform(action)
            }
        },
        //// Смена приоритета в переменной
        TasksMsg::PriorityEditChange(req) => todo.edit.priority = req,
        ////Изменение задачи
        TasksMsg::EditTask => {
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
                            super::handle(todo, Message::ChangePage(Page::TasksList));
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