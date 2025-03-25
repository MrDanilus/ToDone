use serde::{Deserialize, Serialize};

pub mod create;
pub mod get;
pub mod save;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task{
    pub id: String,
    pub name: String,
    pub description: String,
    pub priority: u8,
    pub completed: bool,
    pub subtasks: Vec<String>
}

pub struct _SubTask{
    name: String,
    description: String,
    completed: bool
}

pub enum _HistoryAction{
    CreatedTask,
    CompletedTask,
    DeletedTask,
    CreatedSubTask,
    CompletedSubTask,
    DeletedSubTask
}
pub struct _HistoryUnit{
    action: _HistoryAction,
    task_id: String,
    date: String
}