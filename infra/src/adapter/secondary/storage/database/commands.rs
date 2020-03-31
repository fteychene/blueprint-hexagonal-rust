use super::schema::tasks;
use diesel::{SqliteConnection, Connection, RunQueryDsl};
use anyhow::{anyhow, Error};
use domain::executor::model::model::{Task, TaskId, TaskStatus};
use std::convert::TryInto;
use crate::diesel::*;
use im::HashMap;


pub const SCHEDULED: &str = "SCHEDULED";
pub const SUCCESS: &str = "SUCCESS";
pub const ERROR: &str = "ERROR";

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

pub fn establish_connection(database_url: &str) -> Result<SqliteConnection, Error> {
    SqliteConnection::establish(database_url)
        .map_err(|err| anyhow!("Error connecting to database {}", err))
}

pub fn create_task(conn: &SqliteConnection, new_task: &Task) -> Result<usize, Error> {
    let insertable_task: DbTask = (new_task, &TaskStatus::Scheduled).into();
    diesel::insert_into(tasks::table)
        .values(&insertable_task)
        .execute(conn)
        .map_err(|err| anyhow!("Error inserting in db : {:?}", err))
}

pub fn get_task(conn: &SqliteConnection, task_id: &TaskId) -> Result<TaskStatus, Error> {
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
#[table_name = "tasks"]
struct TaskStatusUpdate<'a> {
    status: &'a str,
    status_log: Option<&'a str>,
}

pub fn update_task(conn: &SqliteConnection, id_value: &str, status: &str, status_log: Option<&str>) -> Result<(), Error> {
    use super::schema::tasks::dsl as tasks_dsl;
    diesel::update(tasks_dsl::tasks.find(id_value))
        .set(&TaskStatusUpdate { status, status_log })
        .execute(conn)
        .map(|_| ())
        .map_err(|err| anyhow!("Error loading from database : {:?}", err))
}

impl From<(&Task, &TaskStatus)> for DbTask {
    fn from(insertable_value: (&Task, &TaskStatus)) -> Self {
        let (task, status) = insertable_value;
        let (status, status_log) = match status {
            TaskStatus::Scheduled => (SCHEDULED.to_string(), None),
            TaskStatus::Success(ref stdout) => (SUCCESS.to_string(), Some(stdout.clone())),
            TaskStatus::Error(ref stderr) => (ERROR.to_string(), Some(stderr.clone())),
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
            SCHEDULED => Ok(TaskStatus::Scheduled),
            SUCCESS => self.status_log.ok_or_else(|| anyhow!("Task {} is defined in database as SUCCES but doesn't have any status_log"))
                .map(|stdout| TaskStatus::Success(stdout)),
            ERROR => self.status_log.ok_or_else(|| anyhow!("Task {} is defined in database as ERROR but doesn't have any status_log"))
                .map(|stdout| TaskStatus::Error(stdout)),
            _ => Err(anyhow!("{} is not a valid status", self.status))
        }?;
        let task = Task {
            id: self.id,
            name: self.name,
            command: self.command,
            env: self.env.map(parse_env_var),
        };
        Ok((task, status))
    }
}

fn parse_env_var(source: String) -> HashMap<String,String> {
    From::from(source.split(";").map(|key_val| {
        let splited: Vec<&str> = key_val.splitn(2, "=").collect();
        match splited.as_slice() {
            [key, value] => (String::from(*key), String::from(*value)),
            _ => (String::from(""), String::from(""))
        }
    }).collect::<Vec<(String, String)>>())
}