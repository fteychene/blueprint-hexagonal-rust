use anyhow::{anyhow, Error, Context};
use im::HashMap;
use im::Vector;

use domain::executor::model::model::{Task, TaskId, TaskStatus};
use domain::executor::ports::secondary::TaskStoragePort;


#[derive(Clone)]
struct StoredTask {
    id: String,
    name: Option<String>,
    command: String,
    env: Option<HashMap<String, String>>,
    status: TaskStatus,
}

pub struct InMemoryStorageAdapter {
    tasks: Vector<StoredTask>
}

impl TaskStoragePort for InMemoryStorageAdapter {
    fn save(&mut self, task: Task) -> Result<Task, Error> {
        self.tasks.push_back(StoredTask::from(&task));
        Ok(task)
    }

    fn status(&mut self, id: TaskId) -> Result<TaskStatus, Error> {
        let kept_id = id.clone();
        self.find_only_one(id).context(format!("Error searching for id {:?}", kept_id))
            .map(|stored_task| stored_task.status.clone())
    }


    fn complete(&mut self, task: &Task, status: TaskStatus) -> Result<(), Error> {
        let id = task.id.clone();
        self.tasks.iter().position(|stored_task| stored_task == TaskId::from(task))
            .map(|index| {
                let mut stored_task = StoredTask::from(task);
                stored_task.status = status;
                self.tasks.set(index, stored_task);
                ()
            }).context(format!("Error completing task {:?}", id))
    }
}

impl InMemoryStorageAdapter {
    pub fn new() -> InMemoryStorageAdapter {
        InMemoryStorageAdapter {
            tasks: Vector::new()
        }
    }

    fn find_only_one(&mut self, id: TaskId) -> Result<&StoredTask, Error> {
        let result: Vec<&StoredTask> = self.tasks.iter()
            .filter(|stored_task| *stored_task == id).collect();
        match result.as_slice() {
            [value] => Ok(value),
            [] =>  Err(anyhow!("No task correspond to your selection")),
            _ =>  Err(anyhow!("More than 1 task correspond to your selection"))
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