mod config;
mod connector;
mod db;
mod files;
mod graph;
mod server;
mod updates;

use anyhow::Context;
use tokio::sync::mpsc;

pub use config::CoordinatorConfig;
use connector::Connectors;

pub async fn run(config: CoordinatorConfig) -> anyhow::Result<()> {
    let uuid = files::init(config.data_dir).context("creating cache files")?;
    let (sender, receiver) = mpsc::channel(config::DEFAULT_CHANNEL_SIZE);
    let connectors = Connectors::new(config.connectors_file, sender).await?;
    server::run_server(config.server, uuid, connectors).await?;
    Ok(())
}
