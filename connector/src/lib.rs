mod config;
mod server;

pub use config::ConnectorConfig;
pub async fn run(config: ConnectorConfig) -> anyhow::Result<()> {
    server::run_server(config.server);
    Ok(())
}
