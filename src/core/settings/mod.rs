use serde::{Deserialize, Serialize};

use crate::ui::styles::ToDoTheme;

pub mod save;
pub mod get;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Settings{
    pub delete_confirm: bool,
    pub theme: Option<ToDoTheme>
}