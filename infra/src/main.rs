extern crate blueprint_hexagonal_domain as domain;
#[macro_use] extern crate diesel;

mod adapter;
mod cli;

use adapter::secondary::storage::database::TaskDatabaseStorageAdapter;
// use adapter::secondary::storage::TaskStorageAdapter;
use adapter::secondary::execution::TaskExecutionAdapter;
use adapter::secondary::id_generator::IdGeneratorAdapter;
use domain::executor::ports::primary::TaskSchedulerPort;
use domain::executor::service::task_execution::TaskScheduler;
use anyhow::Error;
use std::borrow::{BorrowMut, Borrow};
use cli::parse_cli_opts;
use std::env;

fn main() -> Result<(), Error> {
    //TODO Load configuration in a proper way
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut storage = TaskDatabaseStorageAdapter::new(&database_url)?;
    let execution = TaskExecutionAdapter::new();
    let id_generator = IdGeneratorAdapter::new();
    let service = TaskScheduler::new(
        storage.borrow_mut(),
        execution.borrow(),
        id_generator.borrow(),
    );
    run(service)
}

fn run(mut port: impl TaskSchedulerPort) -> Result<(), Error> {
    parse_cli_opts()
        .and_then(|input| port.schedule_task(input))
        .and_then(|task_id| port.task_status(task_id))
        .map(|status| println!("Task status is {:?}", status))
        .map(|_| ())
}