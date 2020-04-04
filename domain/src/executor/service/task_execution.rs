use crate::executor::ports::secondary::{TaskStoragePort, TaskExecutionPort, IdGeneratorPort};
use crate::executor::ports::primary::{TaskSchedulerPort, TaskInput};
use crate::executor::model::model::{Task, TaskId, TaskStatus};
use anyhow::{Error, Context};

pub struct TaskScheduler<'a> {
    storage: &'a mut dyn TaskStoragePort,
    execution: &'a dyn TaskExecutionPort,
    id_generator: &'a dyn IdGeneratorPort,
}

impl TaskSchedulerPort for TaskScheduler<'_> {
    fn schedule_task<T>(&mut self, input_task: T) -> Result<TaskId, Error>
        where T: Into<TaskInput> {
        self.storage.save(task(input_task.into(), self.id_generator.generate_id())).context("Error storing task during schedule")
            // No rule logic for the moment, execute after
            .and_then(|into_task| execute_task(into_task.into(), self.execution, self.storage)).context("Error during task execution")
    }

    fn task_status<T>(&mut self, id: T) -> Result<TaskStatus, Error>
        where T: Into<TaskId> {
        self.storage.status(id.into()).context("Error on task status")
    }
}

impl TaskScheduler<'_> {
    pub fn new<'a>(storage: &'a mut dyn TaskStoragePort, execution: &'a dyn TaskExecutionPort, id_generator: &'a dyn IdGeneratorPort) -> TaskScheduler<'a> {
        TaskScheduler {
            storage,
            execution,
            id_generator,
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
            match storage.complete(&task, TaskStatus::Error(error.to_string())) {
                Ok(_) => error.context(format!("Error during task {} execution", task.id)),
                Err(err) => err.context(format!("Error executing task {} and during status save execution", task.id))
            }
        })
        .and_then(|result| storage.complete(&task, result))
        .map(|_| TaskId::from(&task))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::model::error::TaskError;
    use mockall::*;
    use crate::executor::ports::secondary::{MockTaskExecutionPort, MockTaskStoragePort, MockIdGeneratorPort};

    // TODO add storage of commands to check num of interaction on tests impls
    #[test]
    fn test_execute_task() {
        let input_task = Task {
            id: "test_id".to_string(),
            name: None,
            command: "ls /home".to_string(),
            env: None,
        };

        let mut execution_mock = MockTaskExecutionPort::new();
        execution_mock.expect_execute()
            .times(1)
            .returning(|_| Ok(TaskStatus::Success("Coucou".to_string())));

        let mut storage_mock = MockTaskStoragePort::new();
        storage_mock.expect_complete()
            .times(1)
            .returning(|_, _| Ok(()));

        assert_eq!(execute_task(input_task, &execution_mock, &mut storage_mock).unwrap(), TaskId::Id("test_id".to_string()));
    }

    #[test]
    fn test_execute_task_with_execution_failure() {
        let mut execution_mock = MockTaskExecutionPort::new();
        execution_mock.expect_execute()
            .times(1)
            .returning(|_| Err(TaskError::CommandError("Cannot move /test/inexistant, file does not exists".to_string())));

        let mut storage_mock = MockTaskStoragePort::new();
        storage_mock.expect_complete()
            .times(1)
            .returning(|_, _| Ok(()));

        let input_task = Task {
            id: "test_id".to_string(),
            name: None,
            command: "mv /test/inexistant".to_string(),
            env: None,
        };
        assert_eq!(format!("{}", execute_task(input_task, &execution_mock, &mut storage_mock).unwrap_err()), "Error during task test_id execution");
    }

    #[test]
    fn test_execute_task_with_execution_failure_and_storage_failure() {
        let mut execution_mock = MockTaskExecutionPort::new();
        execution_mock.expect_execute()
            .times(1)
            .returning(|_| Err(TaskError::CommandError("Cannot move /test/inexistant, file does not exists".to_string())));

        let mut storage_mock = MockTaskStoragePort::new();
        storage_mock.expect_complete()
            .times(1)
            .returning(|_, _| Err(anyhow!("Storage failed")));

        let input_task = Task {
            id: "test_id".to_string(),
            name: None,
            command: "mv /test/inexistant".to_string(),
            env: None,
        };
        assert_eq!(format!("{}", execute_task(input_task, &execution_mock, &mut storage_mock).unwrap_err()), "Error executing task test_id and during status save execution");
    }

    #[test]
    fn test_execute_task_with_execution_success_and_storage_failure() {
        let mut execution_mock = MockTaskExecutionPort::new();
        execution_mock.expect_execute()
            .times(1)
            .returning(|_| Ok(TaskStatus::Success("Coucou".to_string())));

        let mut storage_mock = MockTaskStoragePort::new();
        storage_mock.expect_complete()
            .times(1)
            .returning(|_, _| Err(anyhow!("Storage failed")));

        let input_task = Task {
            id: "test_id".to_string(),
            name: None,
            command: "mv /test/inexistant".to_string(),
            env: None,
        };
        assert_eq!(format!("{}", execute_task(input_task, &execution_mock, &mut storage_mock).unwrap_err()), "Storage failed");
    }

    #[test]
    fn test_task_scheduler_schedule_task_should_execute_just_after() {
        let mut execution_mock = MockTaskExecutionPort::new();
        execution_mock.expect_execute()
            .times(1)
            .returning(|_| Ok(TaskStatus::Success("Coucou".to_string())));

        let mut storage_mock = MockTaskStoragePort::new();
        storage_mock.expect_save()
            .times(1)
            .returning(|x| Ok(x));
        storage_mock.expect_complete()
            .times(1)
            .returning(|_, _| Ok(()));

        let mut id_mock = MockIdGeneratorPort::new();
        id_mock.expect_generate_id()
            .times(1)
            .returning(|| "test_id".to_string());

        let mut service = TaskScheduler::new(&mut storage_mock, &execution_mock, &id_mock);

        let input_task = TaskInput {
            name: None,
            command: "ls /home".to_string(),
            env: None
        };
        assert_eq!(service.schedule_task(input_task).unwrap(), TaskId::Id("test_id".to_string()));
    }
}