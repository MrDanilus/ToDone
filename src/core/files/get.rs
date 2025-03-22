use std::fs;

use super::resolve_path;

pub fn get_file(path: &str) -> Result<String, String>{
    let path = match resolve_path(path){
        Ok(path) => path,
        Err(err) => return Err(err)
    };

    let content = match fs::read(path){
        Ok(res) => res,
        Err(err) => return Err(err.to_string())
    };
    return match String::from_utf8(content){
        Ok(res) => Ok(res),
        Err(err) => return Err(err.to_string())
    };
}