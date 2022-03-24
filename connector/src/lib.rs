mod config;
mod files;
// mod gossip;
#[cfg(feature = "lnd")]
mod lnd;
mod server;
mod update;

pub mod node_client;

use anyhow::Context;
pub use config::ConnectorConfig;
use node_client::NodeClient;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn run(config: ConnectorConfig) -> anyhow::Result<()> {
    println!("Starting connector...");
    let mut client = match config.connector.r#type.as_ref() {
        #[cfg(feature = "lnd")]
        "lnd" => lnd::run(config.connector.lnd).await?,
        _ => anyhow::bail!("Connector type not supported"),
    };
    let node_pubkey = client.node_pubkey().await?;
    let uuid = files::init(config.data_dir, &node_pubkey).context("creating cache files")?;
    server::run_server(
        config.server,
        uuid,
        node_pubkey,
        Arc::new(RwLock::new(client)),
    )
    .await?;
    Ok(())
}
