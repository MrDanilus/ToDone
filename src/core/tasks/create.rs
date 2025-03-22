use serde_json::{json, Value};
use uuid::Uuid;

use crate::{core::files, ui::{Message, ToDo}};
use super::Task;

pub fn task(todo: &ToDo){
    let uuid = Uuid::now_v7();
    let task = Task{
        id: uuid.clone().to_string(),
        name: todo.create.name.clone(),
        description: todo.create.description.text().trim().to_string(),
        priority: todo.create.priority,
        completed: false,
        subtasks: Vec::new(),
    };

    let tasks = match files::get::get_file("tasks.todo"){
        Ok(res) => res,
        Err(err) => return { Message::Panic(err); }
    };
    let mut tasks_json = match serde_json::from_str::<Value>(&tasks){
        Ok(res) => res,
        Err(err) => return { Message::Panic(err.to_string()); }
    };
    let tasks_map = match tasks_json.as_object_mut(){
        Some(res) => res,
        None => return { Message::Panic(String::from("Ошибка при создании задачи")); }
    };

    tasks_map.insert(uuid.to_string(), json!(task));
    match files::save::save_file("tasks.todo", tasks_json.to_string().as_bytes()) {
        Ok(_) => {},
        Err(err) => return { Message::Panic(err); }
    }
}