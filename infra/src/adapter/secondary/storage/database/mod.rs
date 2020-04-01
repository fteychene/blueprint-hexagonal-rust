use diesel::SqliteConnection;
use anyhow::Error;
use domain::executor::model::model::{Task, TaskId, TaskStatus};
use domain::executor::ports::secondary::TaskStoragePort;

mod schema;
mod commands;

pub struct SqliteStorageAdapter {
    connection: SqliteConnection
}

impl TaskStoragePort for SqliteStorageAdapter {
    fn save(&mut self, task: Task) -> Result<Task, Error> {
        commands::create_task(&self.connection, &task)
            .map(|_| task)
    }

    fn status(&mut self, id: TaskId) -> Result<TaskStatus, Error> {
        commands::get_task(&self.connection, &id)
    }

    fn complete(&mut self, task: &Task, status: TaskStatus) -> Result<(), Error> {
        match status {
            TaskStatus::Scheduled => commands::update_task(&self.connection, task.id.as_str(), commands::SCHEDULED, None),
            TaskStatus::Success(stdout) => commands::update_task(&self.connection, task.id.as_str(), commands::SUCCESS, Some(stdout.as_str())),
            TaskStatus::Error(stderr) => commands::update_task(&self.connection, task.id.as_str(), commands::ERROR, Some(stderr.as_str())),
        }
    }
}

impl SqliteStorageAdapter {
    pub fn new(database_url: &str) -> Result<SqliteStorageAdapter, Error> {
        let database_connection = commands::establish_connection(database_url)?;
        Ok(SqliteStorageAdapter {
            connection: database_connection
        })
    }
}