use std::collections::HashMap;
use serde_json::json;

use crate::core::functions::files::save;
use super::Task;

pub fn save_all(tasks: HashMap<String, Task>) -> Result<(), String>{
    if let Err(err) = save::save_file("tasks.todo", json!(tasks).to_string().as_bytes()){
        return Err(err)
    }
    return Ok(());
}