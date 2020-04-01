pub mod database;
pub mod memory;

use domain::executor::ports::secondary::TaskStoragePort;
use crate::adapter::secondary::storage::database::SqliteStorageAdapter;
use crate::adapter::secondary::storage::memory::InMemoryStorageAdapter;

pub enum StorageType<'a> {
    Database {
        database_url: &'a str
    },
    InMemory
}

pub fn new_storage_adapter(storage_type: StorageType) -> Result<impl TaskStorageAdapter, Error> {
    match storage_type {
        StorageType::Database { database_url} => SqliteStorageAdapter::new(database_url),
        StorageType::InMemory => Ok(InMemoryStorageAdapter::new())
    }
}