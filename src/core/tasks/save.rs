use std::collections::HashMap;
use serde_json::json;

use crate::core::files::save::save_file;
use super::Task;

pub fn save_all(tasks: HashMap<String, Task>) -> Result<(), String>{
    if let Err(err) = save_file("tasks.todo", json!(tasks).to_string().as_bytes()){
        return Err(err)
    }
    return Ok(());
}