mod background;
mod bitcoind;
mod chain_monitor;
mod invoice_payer;
mod keys;
mod ldk_events;
mod persistence;
mod scorer;

pub mod channel_manager;
pub mod hex_utils;
pub mod logger;
pub mod network_graph;
pub mod peers;

use crate::{
    config::Config, message_forwarder::MessageForwarder,
    uncertainty_graph::UncertaintyGraphMsgForwarder,
};
use anyhow::Context;
use lightning_background_processor::BackgroundProcessor;
use lightning_block_sync::{poll, SpvClient, UnboundedCache};
use network_graph::LnGraph;
use std::{fs, ops::Deref, process, sync::Arc, time::Duration};

pub struct Handles {
    pub background_processor: BackgroundProcessor,
    pub peer_manager: Arc<peers::LnCtlPeers>,
    pub channel_manager: Arc<channel_manager::LnCtlChannelManager>,
    pub network_graph: Arc<LnGraph>,
}

pub async fn run_node(
    config: Config,
    forwarder: UncertaintyGraphMsgForwarder,
) -> anyhow::Result<Handles> {
    fs::create_dir_all(&config.data_dir).context("failed to create data dir")?;
    fs::write(
        format!("{}/pid", config.data_dir.display()),
        process::id().to_string(),
    )
    .context("Could not write pid file")?;
    let announced_node_name = config.node.announced_node_name();
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

    let (network_gossip, network_graph) = network_graph::init_network_graph(
        bitcoind_client.network,
        &config.data_dir,
        Arc::clone(&logger),
    )?;

    let message_forwarder = Arc::new(MessageForwarder::new(
        Arc::clone(&network_gossip),
        forwarder,
    ));

    let peer_manager = peers::init_peer_manager(
        config.node.listen_port,
        Arc::clone(&channel_manager),
        Arc::clone(&message_forwarder),
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
            if let Err(e) = spv_client.poll_best_tip().await {
                eprintln!("Error polling best tip: {:?}", e);
            }
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
        Arc::clone(&channel_manager),
        network_gossip,
        Arc::clone(&peer_manager),
        logger,
    );

    // Regularly reconnect to channel peers.
    let connect_cm = Arc::clone(&channel_manager);
    let connect_pm = Arc::clone(&peer_manager);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            match persistence::read_channel_peer_data(&config.data_dir) {
                Ok(info) => {
                    let peers = connect_pm.get_peer_node_ids();
                    for node_id in connect_cm
                        .list_channels()
                        .iter()
                        .map(|chan| chan.counterparty.node_id)
                        .filter(|id| !peers.contains(id))
                    {
                        for (pubkey, peer_addr) in info.iter() {
                            if *pubkey == node_id {
                                let _ = peers::do_connect_peer(
                                    *pubkey,
                                    *peer_addr,
                                    Arc::clone(&connect_pm),
                                )
                                .await;
                            }
                        }
                    }
                }
                Err(e) => println!(
                    "ERROR: errored reading channel peer info from disk: {:?}",
                    e
                ),
            }
        }
    });

    // Regularly broadcast our node_announcement. This is only required (or possible) if we have
    // some public channels, and is only useful if we have public listen address(es) to announce.
    // In a production environment, this should occur only after the announcement of new channels
    // to avoid churn in the global network graph.
    let chan_manager = Arc::clone(&channel_manager);
    if let Some(net_addr) = config.node.net_address {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                chan_manager.broadcast_node_announcement(
                    [0; 3],
                    announced_node_name,
                    vec![net_addr.clone()],
                );
            }
        });
    }

    for (pubkey, addr) in config.peers {
        let _ = peers::connect_peer_if_necessary(pubkey, addr, Arc::clone(&peer_manager));
    }

    Ok(Handles {
        background_processor,
        peer_manager,
        channel_manager,
        network_graph,
    })
}
