use std::collections::HashMap;
use serde_json::Value;

use crate::core::functions::files::get;
use super::Task;

pub fn get_all() -> Result<HashMap<String, Task>, String>{
    let mut result = HashMap::new();

    let tasks_str = match get::get_file("tasks.todo"){
        Ok(res) => res,
        Err(err) => return Err(err)
    };

    let tasks_json = match serde_json::from_str::<Value>(&tasks_str){
        Ok(res) => res,
        Err(err) => return Err(err.to_string())
    };
    drop(tasks_str);

    let tasks = match tasks_json.as_object(){
        Some(res) => res,
        None => return Err(String::from("Ошибка сериализации задач"))
    };
    for task in tasks{
        match serde_json::from_value::<Task>(task.1.clone()){
            Ok(res) => result.insert(task.0.clone(), res),
            Err(err) => return Err(err.to_string())
        };
    }
    return Ok(result);
}