use anyhow::*;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "bitcoind")]
    pub bitcoind_config: BitcoindConfig,
    pub data_dir: PathBuf,
    pub listen_port: u16,
}

impl Config {
    pub fn from_path(path: PathBuf) -> Result<Self, anyhow::Error> {
        let config_file = std::fs::read_to_string(path).context("Couldn't read config file")?;
        let config: Config =
            serde_yaml::from_str(&config_file).context("Couldn't parse config file")?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct BitcoindConfig {
    pub rpc_user: String,
    pub rpc_password_file: PathBuf,
    pub rpc_host: String,
    pub rpc_port: u16,
    pub network: bitcoin::Network,
}
