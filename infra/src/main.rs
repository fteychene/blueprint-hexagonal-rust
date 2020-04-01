extern crate blueprint_hexagonal_domain as domain;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::borrow::Borrow;

use anyhow::Error;
use itertools::Itertools;

use domain::executor::model::model::{TaskId, TaskStatus};
use domain::executor::ports::primary::TaskSchedulerPort;
use domain::executor::service::task_execution::TaskScheduler;

use crate::primary::cli::{CliOpt, parse_cli_opts, TaskRunOpt, TaskStatusOpt};
use crate::secondary::adapter::execution::LocalExecutionAdapter;
use crate::secondary::adapter::id_generator::UUIDGeneratorAdapter;
use crate::secondary::adapter::storage::new_storage_adapter;


mod secondary;
mod primary;

fn main() -> Result<(), Error> {
    let configuration = primary::settings::load_settings()?;

    let mut storage = new_storage_adapter(configuration.storage)?;
    let execution = LocalExecutionAdapter::new();
    let id_generator = UUIDGeneratorAdapter::new();
    let service = TaskScheduler::new(
        storage.as_mut(),
        execution.borrow(),
        id_generator.borrow(),
    );
    run(service)
}

fn run(mut port: impl TaskSchedulerPort) -> Result<(), Error> {
    match parse_cli_opts() {
        CliOpt::Run(ref task_run_input) => port.schedule_task::<TaskRunOpt>(task_run_input.clone())
            .map(|result|
                if task_run_input.wait {
                    if let Err(err) = port.task_status(result).map(display_task_status) {
                        eprintln!("Error waiting status of task : {:?}", err)
                    };
                } else {
                    match result {
                        TaskId::Id(id) => println!("Task with id {} scheduled", id),
                        TaskId::Name(name) => println!("Task with name {} scheduled", name),
                    }
                }
            ),
        CliOpt::Status(task_status_input) => port.task_status::<TaskStatusOpt>(task_status_input)
            .map(display_task_status)
    }
}

fn display_task_status(status: TaskStatus) {
    match status {
        TaskStatus::Success(stdout) => println!("Task was successfully run :\n {}", stdout.lines().into_iter().map(|line| format!("\t{}", line)).join("\n")),
        TaskStatus::Scheduled => println!("Task is scheduled"),
        TaskStatus::Error(stderr) => eprintln!("Task was in error  :\n {}", stderr.lines().map(|line| format!("\t{}", line)).join("\n"))
    }
}