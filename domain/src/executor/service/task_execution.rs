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
    fn schedule_task<T>(&mut self, input_task: T) -> Result<TaskId, Error>
        where T: Into<TaskInput> {
        self.storage.save(task(input_task.into(), self.id_generator.generate_id()))
            // No rule logic for the moment, execute after
            .and_then(|into_task| execute_task(into_task.into(), self.execution, self.storage))
    }

    fn task_status<T>(&mut self, id: T) -> Result<TaskStatus, Error>
        where T: Into<TaskId> {
        self.storage.status(id.into())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::model::error::TaskError;

    // TODO add storage of commands to check num of interaction on tests impls
    #[test]
    fn test_execute_task() {
        struct TestImplExecution {};
        impl TaskExecutionPort for TestImplExecution {
            fn execute(&self, task: &Task) -> Result<TaskStatus, TaskError> { Ok(TaskStatus::Success("Coucou".to_string())) }
        }
        struct ImplTaskStoragePort{};
        impl TaskStoragePort for ImplTaskStoragePort {
            fn save(&mut self, task: Task) -> Result<Task, Error> { unimplemented!() }
            fn status(&mut self, id: TaskId) -> Result<TaskStatus, Error> { unimplemented!() }
            fn complete(&mut self, task: &Task, status: TaskStatus) -> Result<(), Error> { Ok(()) }
        }
        let input_task = Task {
            id: "test_id".to_string(),
            name: None,
            command: "ls /home".to_string(),
            env: None,
        };
        assert_eq!(execute_task(input_task, &TestImplExecution{}, &mut ImplTaskStoragePort{}).unwrap(), TaskId::Id("test_id".to_string()));
    }

    #[test]
    fn test_execute_task_with_execution_failure() {
        struct TestImplExecution {};
        impl TaskExecutionPort for TestImplExecution {
            fn execute(&self, task: &Task) -> Result<TaskStatus, TaskError> { Err(TaskError::CommandError("Cannot move /test/inexistant, file does not exists".to_string())) }
        }
        struct ImplTaskStoragePort{};
        impl TaskStoragePort for ImplTaskStoragePort {
            fn save(&mut self, task: Task) -> Result<Task, Error> { unimplemented!() }
            fn status(&mut self, id: TaskId) -> Result<TaskStatus, Error> { unimplemented!() }
            fn complete(&mut self, task: &Task, status: TaskStatus) -> Result<(), Error> { Ok(()) }
        }
        let input_task = Task {
            id: "test_id".to_string(),
            name: None,
            command: "mv /test/inexistant".to_string(),
            env: None,
        };
        assert_eq!(format!("{}", execute_task(input_task, &TestImplExecution{}, &mut ImplTaskStoragePort{}).unwrap_err()), "Error during task test_id execution");
    }

    #[test]
    fn test_execute_task_with_execution_failure_and_storage_failure() {
        struct TestImplExecution {};
        impl TaskExecutionPort for TestImplExecution {
            fn execute(&self, task: &Task) -> Result<TaskStatus, TaskError> { Err(TaskError::CommandError("Cannot move /test/inexistant, file does not exists".to_string())) }
        }
        struct ImplTaskStoragePort{};
        impl TaskStoragePort for ImplTaskStoragePort {
            fn save(&mut self, task: Task) -> Result<Task, Error> { unimplemented!() }
            fn status(&mut self, id: TaskId) -> Result<TaskStatus, Error> { unimplemented!() }
            fn complete(&mut self, task: &Task, status: TaskStatus) -> Result<(), Error> { Err(anyhow!("Storage failed")) }
        }
        let input_task = Task {
            id: "test_id".to_string(),
            name: None,
            command: "mv /test/inexistant".to_string(),
            env: None,
        };
        assert_eq!(format!("{}", execute_task(input_task, &TestImplExecution{}, &mut ImplTaskStoragePort{}).unwrap_err()), "Error executing task test_id and during status save execution");
    }

}