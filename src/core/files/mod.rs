pub mod get;
pub mod save;

use std::{env::var, fs, path::{Path, PathBuf}};

pub fn resolve_path(file: &str) -> Result<PathBuf, String>{
    #[cfg(target_family="unix")]
    let path = Path::new("~/.airfish/todo/");
    #[cfg(target_family="unix")]
    return Ok(path.join(&file));

    #[cfg(target_family="windows")]
    let local_app_data = match var("LocalAppData"){
        Ok(res) => res,
        Err(err) => return Err(err.to_string())
    };
    #[cfg(target_family="windows")]
    let path = Path::new(&local_app_data);
    #[cfg(target_family="windows")]
    return Ok(path.join(format!(".airfish\\todo\\{file}")));
}

pub fn check_files() -> Result<(), String>{
    let path = match resolve_path(""){
        Ok(path) => path,
        Err(err) => return Err(err)
    };

    if !path.exists(){
        if let Err(err) = fs::create_dir_all(path){
            return Err(err.to_string())
        }
    };

    // Tasks file
    let tasks_path = resolve_path("tasks.todo").unwrap();
    if !tasks_path.exists(){
        if let Err(err) = fs::write(tasks_path, b"{}"){
            return Err(err.to_string())
        }
    };
    // Subtasks file
    let subtasks_path = resolve_path("subtasks.todo").unwrap();
    if !subtasks_path.exists(){
        if let Err(err) = fs::write(subtasks_path, b"{}"){
            return Err(err.to_string())
        }
    };
    // History file
    let history_path = resolve_path("history.todo").unwrap();
    if !history_path.exists(){
        if let Err(err) = fs::write(history_path, b"{}"){
            return Err(err.to_string())
        }
    };
    return Ok(())
}