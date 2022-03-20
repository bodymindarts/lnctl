use crate::{client, config::Config, grpc, node, uncertainty_graph};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    Server {},
    ListPeers {},
    NodeStatus {},
    Graph {},
}

const DEFAULT_CONFIG: &str = "lnctl.yml";

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG));
    let config = Config::from_path(config_path)?;

    match cli.command {
        Commands::Server {} => {
            run_server(config).await?;
        }
        Commands::ListPeers {} => {
            client::list_peers(config).await?;
        }
        Commands::NodeStatus {} => {
            client::get_node_status(config).await?;
        }
        Commands::Graph {} => {
            client::get_network_graph(config).await?;
        }
    }

    Ok(())
}

async fn run_server(config: Config) -> anyhow::Result<()> {
    let grpc_port = config.grpc_port;
    let (graph_pool, forwarder) = uncertainty_graph::init_uncertainty_graph();
    let handles = node::run_node(config, forwarder).await?;

    grpc::start_server(
        grpc_port,
        handles.peer_manager,
        handles.channel_manager,
        graph_pool,
    )
    .await?;

    handles.background_processor.stop()?;
    Ok(())
}
