use crate::{config::Config, node};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Server {},
}

const DEFAULT_CONFIG: &str = "lnctl.yml";

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG));
    let config = Config::from_path(config_path)?;

    node::run_node(config).await?;

    // // Step 14: Connect and Disconnect Blocks
    // if chain_tip.is_none() {
    //     chain_tip = Some(
    //         init::validate_best_block_header(&mut bitcoind_client.deref())
    //             .await
    //             .unwrap(),
    //     );
    // }
    // let channel_manager_listener = channel_manager.clone();
    // let chain_monitor_listener = chain_monitor.clone();
    // let bitcoind_block_source = bitcoind_client.clone();
    // let network = args.network;
    // tokio::spawn(async move {
    //     let mut derefed = bitcoind_block_source.deref();
    //     let chain_poller = poll::ChainPoller::new(&mut derefed, network);
    //     let chain_listener = (chain_monitor_listener, channel_manager_listener);
    //     let mut spv_client = SpvClient::new(
    //         chain_tip.unwrap(),
    //         chain_poller,
    //         &mut cache,
    //         &chain_listener,
    //     );
    //     loop {
    //         spv_client.poll_best_tip().await.unwrap();
    //         tokio::time::sleep(Duration::from_secs(1)).await;
    //     }
    // });

    // // Step 15: Handle LDK Events
    // let channel_manager_event_listener = channel_manager.clone();
    // let keys_manager_listener = keys_manager.clone();
    // // TODO: persist payment info to disk
    // let inbound_payments: PaymentInfoStorage = Arc::new(Mutex::new(HashMap::new()));
    // let outbound_payments: PaymentInfoStorage = Arc::new(Mutex::new(HashMap::new()));
    // let inbound_pmts_for_events = inbound_payments.clone();
    // let outbound_pmts_for_events = outbound_payments.clone();
    // let network = args.network;
    // let bitcoind_rpc = bitcoind_client.clone();
    // let handle = tokio::runtime::Handle::current();
    // let event_handler = move |event: &Event| {
    //     handle.block_on(handle_ldk_events(
    //         channel_manager_event_listener.clone(),
    //         bitcoind_rpc.clone(),
    //         keys_manager_listener.clone(),
    //         inbound_pmts_for_events.clone(),
    //         outbound_pmts_for_events.clone(),
    //         network,
    //         event,
    //     ));
    // };

    // // Step 16: Initialize routing Scorer
    // let scorer_path = format!("{}/scorer", ldk_data_dir.clone());
    // let scorer = Arc::new(Mutex::new(disk::read_scorer(Path::new(&scorer_path))));
    // let scorer_persist = Arc::clone(&scorer);
    // tokio::spawn(async move {
    //     let mut interval = tokio::time::interval(Duration::from_secs(600));
    //     loop {
    //         interval.tick().await;
    //         if disk::persist_scorer(Path::new(&scorer_path), &scorer_persist.lock().unwrap())
    //             .is_err()
    //         {
    //             // Persistence errors here are non-fatal as channels will be re-scored as payments
    //             // fail, but they may indicate a disk error which could be fatal elsewhere.
    //             eprintln!("Warning: Failed to persist scorer, check your disk and permissions");
    //         }
    //     }
    // });

    // // Step 17: Create InvoicePayer
    // let router = DefaultRouter::new(network_graph.clone(), logger.clone());
    // let invoice_payer = Arc::new(InvoicePayer::new(
    //     channel_manager.clone(),
    //     router,
    //     scorer.clone(),
    //     logger.clone(),
    //     event_handler,
    //     payment::RetryAttempts(5),
    // ));

    // // Step 18: Persist ChannelManager
    // let data_dir = ldk_data_dir.clone();
    // let persist_channel_manager_callback =
    //     move |node: &ChannelManager| FilesystemPersister::persist_manager(data_dir.clone(), &*node);

    // // Step 19: Background Processing
    // let background_processor = BackgroundProcessor::start(
    //     persist_channel_manager_callback,
    //     invoice_payer.clone(),
    //     chain_monitor.clone(),
    //     channel_manager.clone(),
    //     Some(network_gossip.clone()),
    //     peer_manager.clone(),
    //     logger.clone(),
    // );

    // // Regularly reconnect to channel peers.
    // let connect_cm = Arc::clone(&channel_manager);
    // let connect_pm = Arc::clone(&peer_manager);
    // let peer_data_path = format!("{}/channel_peer_data", ldk_data_dir.clone());
    // tokio::spawn(async move {
    //     let mut interval = tokio::time::interval(Duration::from_secs(1));
    //     loop {
    //         interval.tick().await;
    //         match disk::read_channel_peer_data(Path::new(&peer_data_path)) {
    //             Ok(info) => {
    //                 let peers = connect_pm.get_peer_node_ids();
    //                 for node_id in connect_cm
    //                     .list_channels()
    //                     .iter()
    //                     .map(|chan| chan.counterparty.node_id)
    //                     .filter(|id| !peers.contains(id))
    //                 {
    //                     for (pubkey, peer_addr) in info.iter() {
    //                         if *pubkey == node_id {
    //                             let _ = cli::do_connect_peer(
    //                                 *pubkey,
    //                                 peer_addr.clone(),
    //                                 Arc::clone(&connect_pm),
    //                             )
    //                             .await;
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => println!(
    //                 "ERROR: errored reading channel peer info from disk: {:?}",
    //                 e
    //             ),
    //         }
    //     }
    // });

    // // Regularly broadcast our node_announcement. This is only required (or possible) if we have
    // // some public channels, and is only useful if we have public listen address(es) to announce.
    // // In a production environment, this should occur only after the announcement of new channels
    // // to avoid churn in the global network graph.
    // let chan_manager = Arc::clone(&channel_manager);
    // let network = args.network;
    // if !args.ldk_announced_listen_addr.is_empty() {
    //     tokio::spawn(async move {
    //         let mut interval = tokio::time::interval(Duration::from_secs(60));
    //         loop {
    //             interval.tick().await;
    //             chan_manager.broadcast_node_announcement(
    //                 [0; 3],
    //                 args.ldk_announced_node_name,
    //                 args.ldk_announced_listen_addr.clone(),
    //             );
    //         }
    //     });
    // }

    // // Start the CLI.
    // cli::poll_for_user_input(
    //     invoice_payer.clone(),
    //     peer_manager.clone(),
    //     channel_manager.clone(),
    //     keys_manager.clone(),
    //     inbound_payments,
    //     outbound_payments,
    //     ldk_data_dir.clone(),
    //     network,
    // )
    // .await;

    // // Stop the background processor.
    // background_processor.stop().unwrap();
    Ok(())
}
