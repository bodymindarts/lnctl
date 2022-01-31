use crate::{
    bitcoind, chain_monitor, channel_manager, config::Config, keys, ln_peers, logger, persistence,
    uncertainty_graph,
};
use std::sync::Arc;

pub async fn run_node(config: Config) -> Result<(), anyhow::Error> {
    // Initialize our bitcoind client.
    let bitcoind_client = bitcoind::init_bitcoind_client(config.bitcoind_config).await?;

    let logger = logger::init_logger();
    let persister = persistence::init_persister(&config.data_dir)?;

    let chain_monitor = chain_monitor::init_chain_monitor(
        Arc::clone(&bitcoind_client),
        Arc::clone(&logger),
        Arc::clone(&persister),
    );
    let keys_manager = keys::init_keys_manager(&config.data_dir)?;

    let mut channel_monitors = persister.read_channelmonitors(Arc::clone(&keys_manager))?;

    let (channel_manager, chain_tip) = channel_manager::init_channel_manager(
        &config.data_dir,
        channel_monitors,
        Arc::clone(&bitcoind_client),
        Arc::clone(&keys_manager),
        Arc::clone(&chain_monitor),
        Arc::clone(&logger),
    )
    .await?;

    let network_gossip = uncertainty_graph::init_uncertainty_graph(
        bitcoind_client.network,
        &config.data_dir,
        Arc::clone(&logger),
    )?;

    let peer_manager = ln_peers::init_peer_manager(
        config.listen_port,
        Arc::clone(&channel_manager),
        Arc::clone(&network_gossip),
        Arc::clone(&keys_manager),
        Arc::clone(&logger),
    );

    Ok(())
}
