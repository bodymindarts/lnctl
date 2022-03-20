use super::{bitcoind::BitcoindClient, logger::LnCtlLogger, persistence};
use bitcoin::blockdata::constants::genesis_block;
use lightning::{routing::network_graph::NetGraphMsgHandler, routing::network_graph::NetworkGraph};
use std::time::Duration;
use std::{path::Path, sync::Arc};

pub(crate) type LnGraph = NetworkGraph;
pub(crate) type ArcNetGraphMsgHandler =
    Arc<NetGraphMsgHandler<Arc<LnGraph>, Arc<BitcoindClient>, Arc<LnCtlLogger>>>;

pub fn init_network_graph(
    network: bitcoin::Network,
    data_dir: &Path,
    _bitcoind_client: Arc<BitcoindClient>,
    logger: Arc<LnCtlLogger>,
) -> anyhow::Result<(ArcNetGraphMsgHandler, Arc<LnGraph>)> {
    let genesis = genesis_block(network).header.block_hash();
    let network_graph_path = format!("{}/network_graph", data_dir.display());
    let network_graph = Arc::new(persistence::read_network(
        Path::new(&network_graph_path),
        genesis,
    ));
    let network_gossip = Arc::new(NetGraphMsgHandler::new(
        Arc::clone(&network_graph),
        None,
        logger,
    ));
    let network_graph_persist = Arc::clone(&network_graph);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(600));
        loop {
            interval.tick().await;
            if persistence::persist_network(Path::new(&network_graph_path), &network_graph_persist)
                .is_err()
            {
                // Persistence errors here are non-fatal as we can just fetch the routing graph
                // again later, but they may indicate a disk error which could be fatal elsewhere.
                eprintln!(
                    "Warning: Failed to persist network graph, check your disk and permissions"
                );
            }
        }
    });
    Ok((network_gossip, network_graph))
}
