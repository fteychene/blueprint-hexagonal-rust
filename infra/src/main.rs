extern crate blueprint_hexagonal_domain as domain;

mod adapter;

use adapter::secondary::storage::TaskStorageAdapter;
use adapter::secondary::execution::TaskExecutionAdapter;
use adapter::secondary::id_generator::IdGeneratorAdapter;
use domain::executor::ports::primary::{TaskSchedulerPort, TaskInput};
use domain::executor::service::task_execution::TaskScheduler;
use anyhow::Error;
use std::borrow::{BorrowMut, Borrow};
use std::fmt::{Display, Formatter};
use domain::executor::model::model::TaskStatus;

fn main() -> Result<(), Error> {
    let mut storage = TaskStorageAdapter::new();
    let execution = TaskExecutionAdapter::new();
    let id_generator = IdGeneratorAdapter::new();
    let mut service = TaskScheduler::new(
        storage.borrow_mut(),
        execution.borrow(),
        id_generator.borrow(),
    );
    run(service)
}

fn run(mut port: impl TaskSchedulerPort) -> Result<(), Error> {
    port.schedule_task(createTask("ls", None))
        .and_then(|task_id| port.task_status(task_id))
        .map(|status| println!("Task status is {:?}", status));
    Ok(())
}

fn createTask(command: &str, name: Option<String>) -> TaskInput {
    TaskInput {
        name: name,
        command: command.to_string(),
        env: None,
    }
}