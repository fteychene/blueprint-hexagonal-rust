use anyhow::Error;

use domain::executor::ports::secondary::TaskStoragePort;

use crate::secondary::adapter::storage::database::SqliteStorageAdapter;
use crate::secondary::adapter::storage::memory::InMemoryStorageAdapter;
use crate::primary::settings::StorageConfiguration;

pub mod database;
pub mod memory;

pub fn new_storage_adapter(storage_type: StorageConfiguration) -> Result<Box<dyn TaskStoragePort>, Error> {
    match storage_type {
        StorageConfiguration::Database { database_url } => {
            // Result<SqliteStorageAdapter, Error>  => Result<Box<SqliteStorageAdapter>, Error> == Result<Box<dyn TaskStoragePort>, Error>
            // Why does it ot work with this code, check type at compile
            // SqliteStorageAdapter::new(&database_url).map(|adapter| Box::new(adapter))
            Ok(Box::new(SqliteStorageAdapter::new(&database_url)?))

        },
        StorageConfiguration::InMemory => Ok(Box::new(InMemoryStorageAdapter::new()))
    }
}