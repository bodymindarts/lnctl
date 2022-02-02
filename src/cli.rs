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
