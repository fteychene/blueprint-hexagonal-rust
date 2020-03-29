use super::schema::tasks;
use domain::executor::model::model::{Task, TaskId, TaskStatus};
use diesel::{SqliteConnection, Connection, RunQueryDsl};
use std::env;
use im::Vector;
use diesel::query_dsl::InternalJoinDsl;
use anyhow::{anyhow, Error};
use domain::executor::ports::secondary::TaskStoragePort;

pub struct TaskDatabaseStorageAdapter {
    connection: SqliteConnection
}

impl TaskStoragePort for TaskDatabaseStorageAdapter {
    fn save(&mut self, task: Task) -> Result<Task, Error> {
        create_post(&self.connection, &task)
            .map(|_| task)
    }

    fn status(&mut self, id: TaskId) -> Result<TaskStatus, Error> {
        Ok(TaskStatus::Success("Impeccable".to_string()))
    }

    fn complete(&mut self, task: &Task, status: TaskStatus) -> Result<(), Error> {
        Ok(())
    }
}

impl TaskDatabaseStorageAdapter {
    pub fn new() -> TaskDatabaseStorageAdapter {
        TaskDatabaseStorageAdapter {
            connection: establish_connection()
        }
    }
}

#[derive(Queryable, Insertable)]
#[table_name = "tasks"]
struct DbTask {
    id: String,
    name: Option<String>,
    command: String,
    env: Option<String>,
    status: String,
    status_log: Option<String>,
}

pub fn establish_connection() -> SqliteConnection {
    //TODO move loading env var and validation in the begining of the program
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        // TODO error management
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_post(conn: &SqliteConnection, new_task: &Task) -> Result<usize, Error> {
    let test: DbTask = From::from(new_task);
    diesel::insert_into(tasks::table)
        .values(&test)
        .execute(conn)
        .map_err(|err| anyhow!("Error inserting in db : {:?}", err))
}

impl From<&Task> for DbTask {
    fn from(task: &Task) -> Self {
        DbTask {
            id: task.id.clone(),
            name: task.name.clone(),
            command: task.command.clone(),
            env: task.env.as_ref().map(|env_vars| env_vars.iter()
                .fold(vec![], |mut acc, (key, value)| {
                    acc.push(format!("{}:{}", key, value));
                    acc
                })
                .join(";")),
            status: "SCHEDULED".to_string(),
            status_log: None,
        }
    }
}