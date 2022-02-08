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

    Ok(())
}
