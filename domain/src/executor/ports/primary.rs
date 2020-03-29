use anyhow::Error;
use im::HashMap;

use crate::executor::model::model::{TaskId, TaskStatus};

pub trait TaskSchedulerPort {
    fn schedule_task<T>(&mut self, input_task: T) -> Result<TaskId, Error>
        where T: Into<TaskInput>;

    fn task_status<T>(&mut self, id: T) -> Result<TaskStatus, Error>
        where T: Into<TaskId> ;
}

pub struct TaskInput {
    pub name: Option<String>,
    pub command: String,
    pub env: Option<HashMap<String, String>>,
}