use crate::{
    bitcoind::BitcoindClient, client, config::Config, grpc, ln_peers::ChainMonitor,
    logger::LnCtlLogger, node,
};
use clap::{Parser, Subcommand};
use lightning::{
    chain,
    ln::{channelmanager::SimpleArcChannelManager, peer_handler::IgnoringMessageHandler},
    routing::network_graph::{NetGraphMsgHandler, NetworkGraph},
};
use lightning_net_tokio::SocketDescriptor;
use std::{path::PathBuf, sync::Arc};

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
    }

    Ok(())
}

async fn run_server(config: Config) -> anyhow::Result<()> {
    let grpc_port = config.grpc_port;
    let handles = node::run_node(config).await?;

    grpc::start_server::<
        SocketDescriptor,
        Arc<LnCtlLogger>,
        Arc<SimpleArcChannelManager<ChainMonitor, BitcoindClient, BitcoindClient, LnCtlLogger>>,
        Arc<
            NetGraphMsgHandler<
                Arc<NetworkGraph>,
                Arc<dyn chain::Access + Send + Sync>,
                Arc<LnCtlLogger>,
            >,
        >,
        Arc<IgnoringMessageHandler>,
    >(grpc_port, handles.peer_manager)
    .await?;

    handles.background_processor.stop()?;
    Ok(())
}
