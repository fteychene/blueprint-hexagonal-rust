use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Error running the command. Logs : \n {0}")]
    CommandError(String),
    #[error("Error executing the command")]
    ExecutionError {
        source: anyhow::Error
    },
    #[error("Unexpected error while processing the command")]
    UnexpectedError {
        source: Box<dyn std::error::Error>
    }
}


unsafe impl Sync for TaskError {

}

unsafe impl Send for TaskError {

}