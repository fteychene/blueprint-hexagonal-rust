use im::HashMap;
use anyhow::Error;

pub struct Task {
    pub id: String,
    pub command: String,
    pub env: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Error running the command. Logs : \n {0}")]
    CommandError(String),
    #[error("Error executing the command")]
    ExecutionError {
        cause: Error
    }
}

pub type TaskResult = Result<TaskError, String>;