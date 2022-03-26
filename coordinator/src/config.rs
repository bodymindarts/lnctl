use serde::Deserialize;
use std::path::PathBuf;

pub(crate) const DEFAULT_CHANNEL_SIZE: usize = 100;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_server_port")]
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinatorConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default = "default_connectors_file")]
    pub connectors_file: PathBuf,
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
}

fn default_server_port() -> u16 {
    5625
}

fn default_data_dir() -> PathBuf {
    PathBuf::from(".lnctl/coordinator")
}

fn default_connectors_file() -> PathBuf {
    let mut data_dir = default_data_dir();
    data_dir.push("connectors");
    data_dir
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: default_server_port(),
        }
    }
}
