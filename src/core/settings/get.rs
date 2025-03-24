use crate::core::files::get::get_file;
use super::Settings;

pub fn load() -> Result<Settings, String>{
    let settings_str = match get_file("settings.todo"){
        Ok(res) => res,
        Err(err) => return Err(err)
    };

    let settings_json = match serde_json::from_str::<Settings>(&settings_str){
        Ok(res) => res,
        Err(err) => return Err(err.to_string())
    };
    drop(settings_str);
    return Ok(settings_json);
}