use domain::executor::ports::secondary::TaskExecutionPort;
use domain::executor::model::model::{Task, TaskStatus};
use domain::executor::model::error::TaskError;

pub struct TaskExecutionAdapter{}

impl TaskExecutionPort for TaskExecutionAdapter {
    fn execute(&self, task: &Task) -> Result<TaskStatus, TaskError> {
        Ok(TaskStatus::Success("Didn't run lol".to_string()))
    }
}

impl TaskExecutionAdapter {
    pub fn new() -> TaskExecutionAdapter {
        TaskExecutionAdapter {}
    }
}