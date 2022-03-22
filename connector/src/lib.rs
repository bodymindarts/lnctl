mod config;
#[cfg(feature = "lnd")]
mod lnd;
mod server;
mod update;

pub use config::ConnectorConfig;
pub async fn run(config: ConnectorConfig) -> anyhow::Result<()> {
    server::run_server(config.server).await?;
    #[cfg(feature = "lnd")]
    lnd::run_lnd(config.connector.lnd).await?;
    Ok(())
}
