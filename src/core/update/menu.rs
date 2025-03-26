use crate::ui::ToDo;
use super::Message;

#[derive(Debug, Clone)]
pub enum MenuMsg{
    OpenGitHub
}

pub fn handle(todo: &mut ToDo, message: MenuMsg) {
    match message {
        MenuMsg::OpenGitHub => if let Err(err) = open::that(
            "https://github.com/MrDanilus/ToDone"
        ){ super::handle(todo, Message::Panic(err.to_string())); }
    }
}