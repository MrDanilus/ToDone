use serde_json::json;

use crate::core::functions::files::save;
use super::Settings;

pub fn save(settings: Settings) -> Result<(), String>{
    if let Err(err) = save::save_file("settings.todo", json!(settings).to_string().as_bytes()){
        return Err(err)
    }
    return Ok(());
}