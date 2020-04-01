use domain::executor::ports::primary::TaskInput;
use domain::executor::model::model::TaskId;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct TaskRunOpt {
    /// Command to be executed by the task
    #[structopt(required = true)]
    command: Vec<String>,
    /// Name of the task for later querying
    #[structopt(short, long)]
    name: Option<String>,
    /// Wait the execution of the task and print status
    #[structopt(short, long)]
    pub wait: bool,
}

#[derive(Debug, StructOpt)]
pub enum TaskStatusOpt {
    Id {
        #[structopt(required = true)]
        id: String
    },
    Name {
        #[structopt(required = true)]
        name: String
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "tasc")]
pub enum CliOpt {
    #[structopt(name = "run")]
    Run(TaskRunOpt),
    #[structopt(name = "status")]
    Status(TaskStatusOpt),
}


pub fn parse_cli_opts() -> CliOpt {
    CliOpt::from_args()
}

impl Into<TaskInput> for TaskRunOpt {
    fn into(self) -> TaskInput {
        TaskInput {
            command: self.command.join(" "),
            name: self.name,
            env: None,
        }
    }
}

impl Into<TaskId> for TaskStatusOpt {
    fn into(self) -> TaskId {
        match self {
            TaskStatusOpt::Id { id } => TaskId::Id(id),
            TaskStatusOpt::Name { name } => TaskId::Name(name)
        }
    }
}