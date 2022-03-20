mod client;
mod macaroon;

pub use client::LndConfig;

use super::uncertainty_graph::UncertaintyGraphMsgForwarder;

pub async fn start_lnd_connector(
    config: LndConfig,
    forwarder: UncertaintyGraphMsgForwarder,
) -> anyhow::Result<()> {
    Ok(())
}
