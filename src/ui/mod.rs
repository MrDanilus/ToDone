pub mod view;
pub mod pages;
pub mod components;
pub mod styles;

use std::collections::HashMap;
use iced::widget::text_editor;

use crate::core::functions::{settings::Settings, tasks::Task};


#[derive(Debug, Clone, Default)]
pub enum Page {
    // Задачи
    #[default]
    TasksList,
    CreateTask,
    EditTask(String),
    // Система
    Menu
}

#[derive(Default)]
pub struct ToDo {
    // Страницы
    pub panic: String,
    pub page: Page,

    // Список задач
    pub search_text: String,
    pub tasks: HashMap<String, Task>,
    pub task_to_delete: String,

    // Новая задача
    pub create: CreateTask,
    // Редактирование задачи
    pub edit: EditTask,
    // Настройки
    pub settings: Settings
}

#[derive(Default)]
pub struct CreateTask{
    pub name: String,
    pub description: text_editor::Content,
    pub priority: u8,
    pub error: String,
    pub success: String
}

#[derive(Default)]
pub struct EditTask{
    pub id: String,
    pub name: String,
    pub description: text_editor::Content,
    pub priority: u8,
    pub error: String,
    pub success: String
}