use anyhow::{anyhow, Error, Context};

use config::Config;
use std::convert::TryFrom;


#[derive(Debug)]
pub enum StorageConfiguration {
    Database {
        database_url: String
    },
    InMemory,
}

#[derive(Debug)]
pub struct Configuration {
    pub storage: StorageConfiguration
}

pub fn load_settings() -> Result<Configuration, Error> {
    let mut settings = Config::default();
    settings
        .merge(config::File::with_name("settings"))?
        .merge(config::Environment::default().separator("_"))?;

    Configuration::try_from(settings)
}

impl TryFrom<Config> for Configuration {
    type Error = Error;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        StorageConfiguration::try_from(value)
            .map(|storage| Configuration { storage })
            .context("Error loading settings")
    }
}

impl TryFrom<Config> for StorageConfiguration {
    type Error = Error;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        match value.get_str("storage")?.as_str() {
            "database" => Ok(StorageConfiguration::Database { database_url: value.get_str("DATABASE_URL").or(value.get_str("database.url"))? }),
            "inmemory" => Ok(StorageConfiguration::InMemory),
            other => Err(anyhow!("{} is not a valid configuration for storage", other))
        }
    }
}