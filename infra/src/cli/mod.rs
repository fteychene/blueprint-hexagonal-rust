use anyhow::Error;
use domain::executor::ports::primary::TaskInput;

pub struct TaskCliInput {
    command: String,
    name: Option<String>,
}

pub fn parse_cli_opts() -> Result<TaskCliInput, Error> {
    Ok(TaskCliInput {
        command: "ls /home".to_string(),
        name: None,
    })
}

impl Into<TaskInput> for TaskCliInput {
    fn into(self) -> TaskInput {
        TaskInput {
            command: self.command,
            name: self.name,
            env: None,
        }
    }
}