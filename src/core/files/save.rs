use std::fs;

use super::resolve_path;

pub fn save_file(path: &str, content: &[u8]) -> Result<(), String>{
    let path = match resolve_path(path){
        Ok(path) => path,
        Err(err) => return Err(err)
    };

    return match fs::write(path, content){
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    };
}