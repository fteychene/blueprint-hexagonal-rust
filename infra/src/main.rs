extern crate blueprint_hexagonal_domain as domain;
#[macro_use] extern crate diesel;

mod adapter;
mod cli;

use adapter::secondary::storage::{new_storage_adapter, StorageType};
use adapter::secondary::storage::database::SqliteStorageAdapter;
use adapter::secondary::execution::LocalExecutionAdapter;
use adapter::secondary::id_generator::UUIDGeneratorAdapter;
use domain::executor::ports::primary::TaskSchedulerPort;
use domain::executor::service::task_execution::TaskScheduler;
use domain::executor::model::model::{TaskId, TaskStatus};
use anyhow::Error;
use std::borrow::{BorrowMut, Borrow};
use cli::{parse_cli_opts, CliOpt, TaskRunOpt, TaskStatusOpt};
use std::env;
use itertools::Itertools;

fn main() -> Result<(), Error> {
    //TODO Load configuration in a proper way
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut storage = new_storage_adapter(StorageType::Database { database_url: &database_url })?;
    let execution = LocalExecutionAdapter::new();
    let id_generator = UUIDGeneratorAdapter::new();
    let service = TaskScheduler::new(
        storage.borrow_mut(),
        execution.borrow(),
        id_generator.borrow(),
    );
    run(service)
}

fn run(mut port: impl TaskSchedulerPort) -> Result<(), Error> {
    match parse_cli_opts() {
        CliOpt::Run(task_run_input) => port.schedule_task::<TaskRunOpt>(task_run_input)
            .map(|result| match result {
                TaskId::Id(id) => println!("Task with id {} scheduled", id),
                TaskId::Name(name) => println!("Task with name {} scheduled", name),
            }),
        CliOpt::Status(task_status_input) => port.task_status::<TaskStatusOpt>(task_status_input)
            .map(|result| match result {
                TaskStatus::Success(stdout) => println!("Task was successfully run :\n {}", stdout.lines().into_iter().map(|line| format!("\t{}", line)).join("\n")),
                TaskStatus::Scheduled => println!("Task is scheduled"),
                TaskStatus::Error(stderr) => eprintln!("Task was in error  :\n {}", stderr.lines().map(|line| format!("\t{}", line)).join("\n"))
            })
    }
}