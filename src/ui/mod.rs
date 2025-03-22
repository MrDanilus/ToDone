pub mod view;
pub mod pages;
pub mod components;
pub mod styles;

use iced::widget::text_editor;

use crate::core::tasks::Task;


#[derive(Debug, Clone, Default)]
pub enum Page {
    #[default]
    TasksList,
    CreateTask
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
    DeleteTask(String),
    // Новая задача
    NameType(String),
    DescriptionType(text_editor::Action),
    ChangePriority(u8),
    CreateTask
}

#[derive(Default)]
pub struct ToDo {
    // Страницы
    pub panic: String,
    pub page: Page,

    // Список задач
    pub search_text: String,
    pub tasks: Vec<Task>,

    // Новая задача
    pub create: CreateTask
}

#[derive(Default)]
pub struct CreateTask{
    pub name: String,
    pub description: text_editor::Content,
    pub priority: u8,
    pub error: String,
    pub success: String
}