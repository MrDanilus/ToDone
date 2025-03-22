use serde_json::{json, Map, Value};

use crate::core::files::{get::get_file, save::save_file};
use super::Task;

pub fn _save_task(task: Task) -> Result<(), String>{
    let tasks_str = match get_file("tasks.todo"){
        Ok(res) => res,
        Err(err) => return Err(err)
    };

    let mut tasks_json = match serde_json::from_str::<Value>(&tasks_str){
        Ok(res) => res,
        Err(err) => return Err(err.to_string())
    };
    drop(tasks_str);

    let tasks_map_json = match tasks_json.as_object_mut(){
        Some(res) => res,
        None => return Err(String::from("Ошибка сериализации задач"))
    };
    
    match tasks_map_json.contains_key(&task.id){
        true => tasks_map_json[&task.id] = json!(task),
        false => return Err(format!("При сохранении задача с ID: {} на найдена", task.id)),
    };
    if let Err(err) = save_file("tasks.todo", json!(tasks_map_json).to_string().as_bytes()){
        return Err(err)
    }
    
    return Ok(());
}

pub fn rewrite_all(tasks: Vec<Task>) -> Result<(), String>{
    let mut res_tasks = Map::new();
    for task in tasks{
        res_tasks.insert(task.id.clone(), json!(task));
    }

    if let Err(err) = save_file("tasks.todo", json!(res_tasks).to_string().as_bytes()){
        return Err(err)
    }
    return Ok(());
}