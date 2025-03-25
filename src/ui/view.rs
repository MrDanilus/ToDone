use iced::Element;

use crate::{core::{functions::files::check_files, update::Message}, ui::ToDo};
use super::{ pages, Page};

pub fn func(todo: &ToDo) -> Element<Message> {
    if let Err(err) = check_files(){ Message::Panic(err); };
    if !todo.panic.is_empty(){
        println!("{}", todo.panic);
        return pages::panic::func(todo);
    }
    return match todo.page{
        Page::TasksList => pages::tasks::list::func(todo),
        Page::CreateTask => pages::tasks::new_task::func(todo),
        Page::EditTask(_) => pages::tasks::edit_task::func(todo),
        Page::Settings => pages::settings::func(todo)
    }
}