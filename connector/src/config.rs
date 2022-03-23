use serde::Deserialize;
use std::path::PathBuf;

pub(crate) const DEFAULT_CHANNEL_SIZE: usize = 100;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_server_port")]
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Connector {
    pub r#type: String,
    #[cfg(feature = "lnd")]
    pub lnd: crate::lnd::LndConnectorConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConnectorConfig {
    #[serde(default)]
    pub server: ServerConfig,
    pub connector: Connector,
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
}

fn default_server_port() -> u16 {
    5626
}

fn default_data_dir() -> PathBuf {
    PathBuf::from(".lnctl/connector")
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: default_server_port(),
        }
    }
}
