mod client;
mod config;

pub(crate) use client::LndClient;
pub use config::LndConnectorConfig;

pub(crate) async fn run(config: LndConnectorConfig) -> anyhow::Result<LndClient> {
    LndClient::new(config).await
}
