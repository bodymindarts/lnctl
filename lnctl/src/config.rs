use anyhow::Context;
use connector::ConnectorConfig;
use coordinator::CoordinatorConfig;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub connector: ConnectorConfig,
    pub coordinator: CoordinatorConfig,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let config_file = std::fs::read_to_string(path).context("Couldn't read config file")?;
        let config: Config =
            serde_yaml::from_str(&config_file).context("Couldn't parse config file")?;
        Ok(config)
    }
}
