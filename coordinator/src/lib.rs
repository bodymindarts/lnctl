mod bitcoind;
mod bus;
mod config;
mod connector;
mod db;
mod files;
mod graph;
mod server;
mod updates;

use anyhow::Context;
use bus::CoordinatorBus;

pub use config::CoordinatorConfig;
use connector::Connectors;

pub async fn run(config: CoordinatorConfig) -> anyhow::Result<()> {
    let uuid = files::init(config.data_dir).context("creating cache files")?;
    let bus = CoordinatorBus::new(config::DEFAULT_CHANNEL_SIZE);
    let connectors = Connectors::new(config.connectors_file, bus).await?;
    server::run_server(config.server, uuid, connectors).await?;
    Ok(())
}
