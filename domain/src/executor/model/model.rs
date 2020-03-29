use im::HashMap;

use crate::executor::model::error::TaskError;

#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub name: Option<String>,
    pub command: String,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug)]
pub enum TaskStatus {
    Scheduled,
    Success(String),
    Error(String),
}

#[derive(Clone, Debug)]
pub enum TaskId {
    Id(String),
    Name(String)
}

impl From<&Task> for TaskId {
    fn from(task: &Task) -> Self {
        if let Some(ref name) = task.name {
            TaskId::Name(name.clone())
        } else {
            TaskId::Id(task.id.clone())
        }
    }
}