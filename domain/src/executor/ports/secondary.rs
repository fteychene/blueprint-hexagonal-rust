use anyhow::Error;

use crate::executor::model::error::TaskError;
use crate::executor::model::model::{Task, TaskId, TaskStatus};

pub trait TaskStoragePort {
    fn save(&mut self, task: Task) -> Result<Task, Error>;

    fn status(&mut self, id: TaskId) -> Result<TaskStatus, Error>;

    fn complete(&mut self, task: &Task, status: TaskStatus) -> Result<(), Error>;

}

pub trait TaskExecutionPort {
    fn execute(&self, task: &Task) -> Result<TaskStatus, TaskError>;
}

pub trait IdGeneratorPort {
    fn generate_id(&self) -> String;
}