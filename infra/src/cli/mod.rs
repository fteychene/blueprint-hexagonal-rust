use anyhow::Error;
use domain::executor::ports::primary::TaskInput;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tasc")]
pub struct TaskCliInput {
    /// Command to be executed by the task
    #[structopt()]
    command: Vec<String>,
    /// Optional : Name of the task for later querying
    #[structopt(short, long)]
    name: Option<String>,
}

pub fn parse_cli_opts() -> Result<TaskCliInput, Error> {
    Ok(TaskCliInput::from_args())
}

impl Into<TaskInput> for TaskCliInput {
    fn into(self) -> TaskInput {
        TaskInput {
            command: self.command.join(" "),
            name: self.name,
            env: None,
        }
    }
}