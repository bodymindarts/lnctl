mod config;
mod files;
mod gossip;
mod identifier;
#[cfg(feature = "lnd")]
mod lnd;
mod server;
mod update;

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

    let node_pubkey = client.node_pubkey().await?;
    let connector_identifier =
        files::init(config.data_dir, &node_pubkey).context("creating cache files")?;
    let receiver = Gossip::listen(config.gossip.port, connector_identifier.secret_key);

    let _ = client
        .connect_to_peer(
            connector_identifier.public_key,
            format!("{}:{}", config.gossip.host, config.gossip.port),
        )
        .await?;

    server::run_server(
        config.server,
        connector_identifier.uuid,
        node_pubkey,
        receiver,
        Arc::new(RwLock::new(client)),
    )
    .await?;
    Ok(())
}
