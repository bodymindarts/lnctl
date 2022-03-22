use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct LndConnectorConfig {
    pub admin_endpoint: String,
    pub cert_path: PathBuf,
    pub macaroon_path: PathBuf,
}
