#![allow(unused)]

use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub debug: bool,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub log: Log,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let c = Config::builder()
            .add_source(File::with_name("config.toml").required(false))
            .build()?;

        c.try_deserialize()
    }
}
