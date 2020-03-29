use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Error running the command. Logs : \n {0}")]
    CommandError(String),
    #[error("Error executing the command")]
    ExecutionError {
        #[from]
        source: anyhow::Error
    }
}