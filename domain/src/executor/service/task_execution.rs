use crate::executor::ports::secondary::{TaskStoragePort, TaskExecutionPort, IdGeneratorPort};
use crate::executor::ports::primary::{TaskSchedulerPort, TaskInput};
use crate::executor::model::model::{Task, TaskId, TaskStatus};
use anyhow::{anyhow, Error};

pub struct TaskScheduler<'a> {
    storage: &'a mut dyn TaskStoragePort,
    execution: &'a dyn TaskExecutionPort,
    id_generator: &'a dyn IdGeneratorPort,
}


impl TaskSchedulerPort for TaskScheduler<'_> {
    fn schedule_task(&mut self, input_task: TaskInput) -> Result<TaskId, Error> {
        self.storage.save(task(input_task.into(), self.id_generator.generate_id()))
            // No rule logic for the moment, execute after
            .and_then(|into_task| execute_task(into_task.into(), self.execution, self.storage))
    }

    fn task_status(&mut self, id: TaskId) -> Result<TaskStatus, Error> {
        self.storage.status(id)
    }
}

impl TaskScheduler <'_> {
    pub fn new<'a>(storage: &'a mut dyn TaskStoragePort, execution: &'a dyn TaskExecutionPort, id_generator: &'a dyn IdGeneratorPort) -> TaskScheduler<'a> {
        TaskScheduler {
            storage,
            execution,
            id_generator
        }
    }
}


fn task(input: TaskInput, id: String) -> Task {
    Task {
        id,
        command: input.command,
        name: input.name,
        env: input.env,
    }
}

fn execute_task(task: Task, executor: &dyn TaskExecutionPort, storage: &mut dyn TaskStoragePort) -> Result<TaskId, Error> {
    executor.execute(&task)
        .map_err(|error| {
            match storage.complete(&task, TaskStatus::Error(format!("{:?}", error))) {
                Ok(_) => anyhow!("Error during task {} execution", task.id),
                Err(err) => {
                    eprintln!("Error saving error status for task {} : {}", task.id, err);
                    anyhow!("Error executing task {} and during status save execution", task.id)
                }
            }
        })
        .and_then(|result| storage.complete(&task, result))
        .map(|_| TaskId::from(&task))
}