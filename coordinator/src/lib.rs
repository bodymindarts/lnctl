mod config;
mod server;

pub use config::CoordinatorConfig;

pub async fn run(config: CoordinatorConfig) -> anyhow::Result<()> {
    server::run_server(config.server).await?;
    Ok(())
}
