use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::{client, config::Config};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Connector {},
    Server {},
    GetStatus {},
    ChannelHistory { channel_id: u64 },
}

const DEFAULT_CONFIG: &str = "lnctl.yml";

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG));
    let config = Config::from_path(config_path)?;

    match cli.command {
        Commands::Connector {} => {
            connector::run(config.connector).await?;
        }
        Commands::Server {} => {
            gateway::run(config.gateway).await?;
        }
        Commands::GetStatus {} => {
            let config = client::ClientConfig {
                addr: "localhost".to_string(),
                port: config.gateway.server.port,
            };

            client::get_status(config).await?;
        }
        Commands::ChannelHistory { channel_id } => {
            let config = client::ClientConfig {
                addr: "localhost".to_string(),
                port: config.gateway.server.port,
            };

            client::channel_history(config, channel_id).await?;
        }
    }
    Ok(())
}
