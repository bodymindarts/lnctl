mod config;
mod server;
mod update;

pub use config::ConnectorConfig;
pub async fn run(config: ConnectorConfig) -> anyhow::Result<()> {
    server::run_server(config.server);
    Ok(())
}
