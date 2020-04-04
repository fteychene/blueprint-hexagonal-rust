use domain::executor::ports::secondary::TaskExecutionPort;
use domain::executor::model::model::{Task, TaskStatus};
use domain::executor::model::error::TaskError;
use std::process::{Command, Output};
use anyhow::{anyhow, Error, Context};
use im::Vector;
use std::iter::FromIterator;

pub struct LocalExecutionAdapter {}

impl TaskExecutionPort for LocalExecutionAdapter {
    fn execute(&self, task: &Task) -> Result<TaskStatus, Error> {
        let command_splitted = Vector::from_iter(task.command.split_whitespace().into_iter());
        let main_command: &str = command_splitted.head()
            .ok_or(TaskError::CommandError("Command can't be empty".to_string()))
            .context("Error during command validation")?;
        Command::new(main_command)
            .args(command_splitted.split_at(1).1)
            .output()
            .map_err(|err| TaskError::ExecutionError { source: anyhow!("{:?}", err) })
            .and_then(|output| validate_output(output))
            .context("Error during command execution")
    }
}

impl LocalExecutionAdapter {
    pub fn new() -> LocalExecutionAdapter {
        LocalExecutionAdapter {}
    }
}

// TaskError -> anyhow::Error
// FromUTF8Error ? -> anyhow::Error
fn validate_output(output: Output) -> Result<TaskStatus, TaskError> {
    match output.status.success() {
        true => String::from_utf8(output.stdout)
            .map_err(|err| TaskError::UnexpectedError { source: Box::new(err) })
            .map(|output_string| TaskStatus::Success(output_string)),
        false => String::from_utf8(output.stderr)
            .map_err(|err| TaskError::UnexpectedError { source: Box::new(err) })
            .and_then(|stderr_string| Err(TaskError::CommandError(stderr_string)))
    }
}