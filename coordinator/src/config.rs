use serde::Deserialize;

pub(crate) const DEFAULT_CHANNEL_SIZE: usize = 10000;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinatorConfig {
    pub server: ServerConfig,
}
