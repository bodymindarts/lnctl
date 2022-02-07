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

    // // Regularly reconnect to channel peers.

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
    Ok(())
}
