use anyhow::Error;
use im::HashMap;

use crate::executor::model::model::{TaskId, TaskStatus};

pub trait TaskSchedulerPort {
    fn schedule_task(&mut self, input_task: TaskInput) -> Result<TaskId, Error>;

    fn task_status(&mut self, id: TaskId) -> Result<TaskStatus, Error>;
}

pub struct TaskInput {
    pub name: Option<String>,
    pub command: String,
    pub env: Option<HashMap<String, String>>,
}