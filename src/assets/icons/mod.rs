// Tasks
pub fn check_icon() -> &'static[u8]{
    return include_bytes!("tasks/check.svg").as_slice()
}
pub fn cross_icon() -> &'static[u8]{
    return include_bytes!("tasks/cross.svg").as_slice()
}
pub fn delete_icon() -> &'static[u8]{
    return include_bytes!("tasks/delete.svg").as_slice()
}
pub fn edit_icon() -> &'static[u8]{
    return include_bytes!("tasks/edit.svg").as_slice()
}

// All
pub fn arrow_back_icon() -> &'static[u8]{
    return include_bytes!("arrow_back.svg").as_slice()
}
pub fn settings_icon() -> &'static[u8]{
    return include_bytes!("settings.svg").as_slice()
}