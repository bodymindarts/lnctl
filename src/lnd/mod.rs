mod client;
mod macaroon;

use client::LndClient;
pub use client::LndConfig;

use super::uncertainty_graph::UncertaintyGraphMsgForwarder;

pub async fn start_lnd_connector(
    config: LndConfig,
    forwarder: UncertaintyGraphMsgForwarder,
) -> anyhow::Result<()> {
    let mut client = LndClient::new(config).await?;
    let channels = client.list_channels().await?;
    println!("channels: {:?}", channels);
    Ok(())
}
