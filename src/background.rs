use crate::{
    chain_monitor::ChainMonitor, channel_manager::LnCtlChannelManager, invoice_payer::InvoicePayer,
    logger::LnCtlLogger, peers::LnCtlPeers, uncertainty_graph::ArcNetGraphMsgHandler,
};
use lightning::util::events::EventHandler;
use lightning_background_processor::BackgroundProcessor;
use lightning_persister::FilesystemPersister;
use std::{path::Path, sync::Arc};

pub(crate) fn start_background_processor<E: EventHandler + Send + Sync + 'static>(
    data_dir: &Path,
    invoice_payer: Arc<InvoicePayer<E>>,
    chain_monitor: Arc<ChainMonitor>,
    channel_manager: Arc<LnCtlChannelManager>,
    network_gossip: ArcNetGraphMsgHandler,
    peer_manager: Arc<LnCtlPeers>,
    logger: Arc<LnCtlLogger>,
) -> BackgroundProcessor {
    let file_name = data_dir.display().to_string();
    let persist_channel_manager_callback = move |node: &LnCtlChannelManager| {
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
