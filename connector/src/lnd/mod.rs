mod client;
mod config;

pub use client::LndClient;
pub use config::LndConnectorConfig;

pub async fn run_lnd(config: LndConnectorConfig) -> anyhow::Result<LndClient> {
    LndClient::new(config).await
}
