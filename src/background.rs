use crate::{
    bitcoind::BitcoindClient, chain_monitor::ChainMonitor, channel_manager::ChannelManager,
    hex_utils, invoice_payer::InvoicePayer, ln_peers::LnPeers, logger::LnCtlLogger,
    uncertainty_graph::ArcNetGraphMsgHandler,
};
use lightning::util::events::EventHandler;
use lightning_background_processor::BackgroundProcessor;
use lightning_persister::FilesystemPersister;
use std::{path::PathBuf, sync::Arc};

pub(crate) fn start_background_processor<E: EventHandler + Send + Sync + 'static>(
    data_dir: &PathBuf,
    invoice_payer: Arc<InvoicePayer<E>>,
    chain_monitor: Arc<ChainMonitor>,
    channel_manager: Arc<ChannelManager>,
    network_gossip: ArcNetGraphMsgHandler,
    peer_manager: Arc<LnPeers>,
    logger: Arc<LnCtlLogger>,
) -> BackgroundProcessor {
    let file_name = data_dir.as_path().display().to_string();
    let persist_channel_manager_callback = move |node: &ChannelManager| {
        FilesystemPersister::persist_manager(file_name.clone(), &*node)
    };

    BackgroundProcessor::start(
        persist_channel_manager_callback,
        invoice_payer,
        chain_monitor,
        channel_manager,
        Some(network_gossip.clone()),
        peer_manager,
        logger,
    )
}
