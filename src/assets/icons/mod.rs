pub fn check_icon() -> &'static[u8]{
    return include_bytes!("check.svg").as_slice()
}

pub fn close_icon() -> &'static[u8]{
    return include_bytes!("close.svg").as_slice()
}

pub fn delete_icon() -> &'static[u8]{
    return include_bytes!("delete.svg").as_slice()
}