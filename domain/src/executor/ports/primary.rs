use crate::executor::model::model::{Task, TaskStatus, TaskId};
use anyhow::Error;

pub trait TaskExecutorPort {
    fn schedule_task(input_task: T) -> Result<TaskId, Error>
        where T: Into<Task>;

    fn task_status(id: T) -> Result<TaskStatus, Error>
        where T: Into<TaskId>;
}