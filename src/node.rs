use crate::{
    background, bitcoind, chain_monitor, channel_manager, config::Config, invoice_payer, keys,
    ldk_events, ln_peers, logger, persistence, scorer, uncertainty_graph,
};
use lightning_block_sync::{poll, SpvClient, UnboundedCache};
use std::{ops::Deref, sync::Arc, time::Duration};

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

    let channel_monitors = persister.read_channelmonitors(Arc::clone(&keys_manager))?;

    let mut cache = UnboundedCache::new();
    let (channel_manager, chain_tip) = channel_manager::init_channel_manager(
        &config.data_dir,
        channel_monitors,
        Arc::clone(&bitcoind_client),
        Arc::clone(&keys_manager),
        Arc::clone(&chain_monitor),
        &mut cache,
        Arc::clone(&logger),
    )
    .await?;

    let (network_gossip, network_graph) = uncertainty_graph::init_uncertainty_graph(
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

    // // Step 14: Connect and Disconnect Blocks
    let channel_manager_listener = Arc::clone(&channel_manager);
    let chain_monitor_listener = Arc::clone(&chain_monitor);
    let bitcoind_block_source = Arc::clone(&bitcoind_client);
    tokio::spawn(async move {
        let mut derefed = bitcoind_block_source.deref();
        let chain_poller = poll::ChainPoller::new(&mut derefed, bitcoind_block_source.network);
        let chain_listener = (chain_monitor_listener, channel_manager_listener);
        let mut spv_client = SpvClient::new(chain_tip, chain_poller, &mut cache, &chain_listener);
        loop {
            spv_client.poll_best_tip().await.unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // Step 15: Handle LDK Events
    let event_handler = ldk_events::init_ldk_events_handler(
        Arc::clone(&bitcoind_client),
        Arc::clone(&keys_manager),
        Arc::clone(&channel_manager),
    );

    // Step 16: Initialize routing Scorer
    let scorer = scorer::init_scorer(&config.data_dir);

    // Step 17: Create InvoicePayer
    let invoice_payer = invoice_payer::init_invoice_payer(
        Arc::clone(&channel_manager),
        Arc::clone(&network_graph),
        Arc::clone(&scorer),
        event_handler,
        Arc::clone(&logger),
    );

    let background_processor = background::start_background_processor(
        &config.data_dir,
        invoice_payer,
        chain_monitor,
        channel_manager,
        network_gossip,
        peer_manager,
        logger,
    );

    background_processor.stop().unwrap();
    Ok(())
}
