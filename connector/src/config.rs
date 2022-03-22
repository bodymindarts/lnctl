use serde::Deserialize;

pub(crate) const DEFAULT_CHANNEL_SIZE: usize = 100;

#[derive(Debug, Clone, Deserialize)]
pub struct TlsConfig {
    pub cert_file: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub tls: TlsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConnectorConfig {
    pub server: ServerConfig,
}
