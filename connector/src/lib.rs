mod config;
mod db;
mod files;
mod gossip;
#[cfg(feature = "lnd")]
mod lnd;
mod primitives;
mod server;

pub mod node_client;

use anyhow::Context;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use config::ConnectorConfig;
use gossip::Gossip;
use node_client::NodeClient;

pub async fn run(config: ConnectorConfig) -> anyhow::Result<()> {
    println!("Starting connector...");
    let mut client = match config.connector.r#type.as_ref() {
        #[cfg(feature = "lnd")]
        "lnd" => lnd::run(config.connector.lnd).await?,
        _ => anyhow::bail!("Connector type not supported"),
    };

    let node_info = client.node_info().await?;
    let (connector_id, connector_pubkey, connector_secret_key) =
        files::init(config.data_dir, &node_info.node_id).context("creating cache files")?;
    let receiver = Gossip::listen(config.gossip.port, node_info.network, connector_secret_key);

    let _ = client
        .connect_to_peer(
            connector_pubkey.into(),
            format!("{}:{}", config.gossip.host, config.gossip.port),
        )
        .await?;

    server::run_server(
        config.server,
        connector_id,
        node_info.node_id,
        receiver,
        Arc::new(RwLock::new(client)),
    )
    .await?;
    Ok(())
}
