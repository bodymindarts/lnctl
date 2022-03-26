mod config;
mod connector;
mod files;
mod server;

use anyhow::Context;
pub use config::CoordinatorConfig;
use connector::Connectors;

pub async fn run(config: CoordinatorConfig) -> anyhow::Result<()> {
    let uuid = files::init(config.data_dir).context("creating cache files")?;
    let connectors = Connectors::new(config.connectors_file).await?;
    server::run_server(config.server, uuid, connectors).await?;
    Ok(())
}
