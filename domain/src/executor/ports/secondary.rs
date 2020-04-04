use anyhow::Error;
use mockall::*;

use crate::executor::model::model::{Task, TaskId, TaskStatus};

#[automock]
pub trait TaskStoragePort {
    fn save(&mut self, task: Task) -> Result<Task, Error>;

    fn status(&mut self, id: TaskId) -> Result<TaskStatus, Error>;

    fn complete(&mut self, task: &Task, status: TaskStatus) -> Result<(), Error>;

}

#[automock]
pub trait TaskExecutionPort {
    fn execute(&self, task: &Task) -> Result<TaskStatus, Error>;
}

#[automock]
pub trait IdGeneratorPort {
    fn generate_id(&self) -> String;
}