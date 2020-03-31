use anyhow::Error;
use im::HashMap;
use im::Vector;

use domain::executor::model::model::{Task, TaskId, TaskStatus};
use domain::executor::ports::secondary::TaskStoragePort;
use anyhow::anyhow;

#[derive(Clone)]
struct StoredTask {
    id: String,
    name: Option<String>,
    command: String,
    env: Option<HashMap<String, String>>,
    status: TaskStatus,
}

pub struct TaskStorageAdapter {
    tasks: Vector<StoredTask>
}

impl TaskStoragePort for TaskStorageAdapter {
    fn save(&mut self, task: Task) -> Result<Task, Error> {
        self.tasks.push_back(StoredTask::from(&task));
        Ok(task)
    }

    fn status(&mut self, id: TaskId) -> Result<TaskStatus, Error> {
        self.tasks.iter()
            .find(|stored_task| *stored_task == id)
            .map(|stored_task| stored_task.status.clone())
            .ok_or_else(|| anyhow!("Can't find task"))
    }

    fn complete(&mut self, task: &Task, status: TaskStatus) -> Result<(), Error> {
        self.tasks.iter().position(|stored_task| stored_task == TaskId::from(task))
            .map(|index| {
                let mut stored_task = StoredTask::from(task);
                stored_task.status = status;
                self.tasks.set(index, stored_task);
                ()
            })
            .ok_or_else(|| anyhow!("Can't find task"))
    }
}

impl TaskStorageAdapter {
    pub fn new() -> TaskStorageAdapter {
        TaskStorageAdapter {
            tasks: Vector::new()
        }
    }
}

impl From<&Task> for StoredTask {
    fn from(task: &Task) -> Self {
        StoredTask {
            id: task.id.clone(),
            name: task.name.clone(),
            command: task.command.clone(),
            env: task.env.clone(),
            status: TaskStatus::Scheduled,
        }
    }
}

impl From<Task> for StoredTask {
    fn from(task: Task) -> Self {
        From::from(&task)
    }
}

impl PartialEq<TaskId> for &StoredTask {
    fn eq(&self, other: &TaskId) -> bool {
       match other {
           TaskId::Id(ref id) => self.id == id.clone(),
           TaskId::Name(ref name) => self.name == Some(name.clone())
       }
    }
}