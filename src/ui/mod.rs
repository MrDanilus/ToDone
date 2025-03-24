pub mod view;
pub mod pages;
pub mod components;
pub mod styles;

use std::collections::HashMap;
use iced::widget::text_editor;
use styles::ToDoTheme;

use crate::core::{settings::Settings, tasks::Task};


#[derive(Debug, Clone, Default)]
pub enum Page {
    // Задачи
    #[default]
    TasksList,
    CreateTask,
    EditTask(String),
    // Система
    Settings
}

#[derive(Debug, Clone)]
pub enum Message {
    // Система
    LoadTasks,

    // Страницы
    Panic(String),
    ChangePage(Page),
    // Список задач
    SearchChange(String),
    CompleteTask(String),
    DeleteConfirm(String),
    DeleteTask(String),

    // Новая задача
    NameCreateType(String),
    DescriptionCreateType(text_editor::Action),
    PriorityCreateChange(u8),
    CreateTask,
    // Редактирование задачи
    NameEditType(String),
    DescriptionEditType(text_editor::Action),
    PriorityEditChange(u8),
    EditTask,
    // Настройки
    LoadSettings,
    ChangeDeleteConfirm,
    ChangeTheme(ToDoTheme)
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