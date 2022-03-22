use serde::Deserialize;

pub(crate) const DEFAULT_CHANNEL_SIZE: usize = 100;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Connector {
    #[cfg(feature = "lnd")]
    pub lnd: crate::lnd::LndConnectorConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConnectorConfig {
    pub server: ServerConfig,
    pub connector: Connector,
}
