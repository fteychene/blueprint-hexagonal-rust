use super::schema::tasks;
use domain::executor::model::model::{Task, TaskId, TaskStatus};
use diesel::{SqliteConnection, Connection, RunQueryDsl};
use std::env;
use anyhow::{anyhow, Error};
use domain::executor::ports::secondary::TaskStoragePort;
use std::convert::TryInto;
use crate::diesel::*;

pub struct TaskDatabaseStorageAdapter {
    connection: SqliteConnection
}

impl TaskStoragePort for TaskDatabaseStorageAdapter {
    fn save(&mut self, task: Task) -> Result<Task, Error> {
        create_task(&self.connection, &task)
            .map(|_| task)
    }

    fn status(&mut self, id: TaskId) -> Result<TaskStatus, Error> {
        get_task(&self.connection, &id)
    }

    // TODO change SCHEDULED, SUCCESS, ERROR to const values
    fn complete(&mut self, task: &Task, status: TaskStatus) -> Result<(), Error> {
        match status {
            TaskStatus::Scheduled => update_task(&self.connection, task.id.as_str(), "SCHEDULED", None),
            TaskStatus::Success(stdout) => update_task(&self.connection, task.id.as_str(), "SUCCESS", Some(stdout.as_str())),
            TaskStatus::Error(stderr) => update_task(&self.connection, task.id.as_str(), "ERROR", Some(stderr.as_str())),
        }
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

fn create_task(conn: &SqliteConnection, new_task: &Task) -> Result<usize, Error> {
    let test: DbTask = (new_task, &TaskStatus::Scheduled).into();
    diesel::insert_into(tasks::table)
        .values(&test)
        .execute(conn)
        .map_err(|err| anyhow!("Error inserting in db : {:?}", err))
}

fn get_task(conn: &SqliteConnection, task_id: &TaskId) -> Result<TaskStatus, Error> {
    use super::schema::tasks::dsl::*;
    let (_, status_value) = match task_id {
        TaskId::Id(id_value) => tasks.filter(id.eq(id_value))
            .limit(1)
            .first::<DbTask>(conn)
            .map_err(|err| anyhow!("Error loading from database : {:?}", err))?.try_into(),
        TaskId::Name(name_value) => tasks.filter(name.nullable().eq(name_value))
            .limit(1)
            .first::<DbTask>(conn)
            .map_err(|err| anyhow!("Error loading from database : {:?}", err))?.try_into()
        }?;
    return Ok(status_value);
}

#[derive(AsChangeset)]
#[table_name="tasks"]
struct TaskStatusUpdate<'a> {
    status: &'a str,
    status_log: Option<&'a str>,
}

fn update_task(conn: &SqliteConnection, id_value: &str, status_value: &str, status_log_value: Option<&str>) -> Result<(), Error> {
    use super::schema::tasks::dsl::*;
    // TODO change import to previx coumn name by table name to clean var names
    diesel::update(tasks.find(id_value))
        .set(&TaskStatusUpdate{ status: status_value, status_log: status_log_value})
        .execute(conn)
        .map(|_| ())
        .map_err(|err| anyhow!("Error loading from database : {:?}", err))
}

impl From<(&Task, &TaskStatus)> for DbTask {
    fn from(insertable_value: (&Task, &TaskStatus)) -> Self {
        let (task, status) = insertable_value;
        let (status, status_log) = match status {
            TaskStatus::Scheduled => ("SCHEDULED".to_string(), None),
            TaskStatus::Success(ref stdout) => ("SUCCESS".to_string(), Some(stdout.clone())),
            TaskStatus::Error(ref stderr) => ("ERROR".to_string(), Some(stderr.clone())),
        };
        DbTask {
            id: task.id.clone(),
            name: task.name.clone(),
            command: task.command.clone(),
            // KEY=VAL;KEY2=VAL2;KEY3=VAL
            env: task.env.as_ref().map(|env_vars| env_vars.iter()
                .fold(vec![], |mut acc, (key, value)| {
                    acc.push(format!("{}:{}", key, value));
                    acc
                })
                .join(";")),
            status,
            status_log,
        }
    }
}

impl TryInto<(Task, TaskStatus)> for DbTask {
    type Error = Error;

    fn try_into(self) -> Result<(Task, TaskStatus), Self::Error> {
        let status = match self.status.as_str() {
            "SCHEDULED" => Ok(TaskStatus::Scheduled),
            "SUCCESS" => self.status_log.ok_or_else(|| anyhow!("Task {} is defined in database as SUCCES but doesn't have any status_log"))
                .map(|stdout| TaskStatus::Success(stdout)),
            "ERROR" => self.status_log.ok_or_else(|| anyhow!("Task {} is defined in database as ERROR but doesn't have any status_log"))
                .map(|stdout| TaskStatus::Error(stdout)),
            _ => Err(anyhow!("{} is not a valid status", self.status))
        }?;

        // Option<Vec<Option<(String, String)>>> => Option<Option<Vec<(String, String)>>> => Option<Vec<(String, String)>> => Option<Vec<(String, String)>> => Option<HashMap<String, String>>
        let task = Task {
            id: self.id,
            name: self.name,
            command: self.command,
            env: self.env.map(|env_value| From::from(env_value.split(";").map(|key_val| {
                let splited: Vec<&str> = key_val.splitn(2, "=").collect();
                match splited.as_slice() {
                    [key, value] => (String::from(*key), String::from(*value)),
                    _ => (String::from(""), String::from(""))
                }
            }).collect::<Vec<(String, String)>>())),
        };
        Ok((task, status))
    }
}