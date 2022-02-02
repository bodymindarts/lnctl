use crate::{
    bitcoind::BitcoindClient, channel_manager::ChannelManager, hex_utils, logger::LnCtlLogger,
};
use lightning::{
    routing::{network_graph::NetworkGraph, scoring::Scorer},
    util::events::EventHandler,
};
use lightning_invoice::{payment, utils::DefaultRouter};
use std::sync::{Arc, Mutex};

pub(crate) type InvoicePayer<E> =
    payment::InvoicePayer<Arc<ChannelManager>, Router, Arc<Mutex<Scorer>>, Arc<LnCtlLogger>, E>;
type Router = DefaultRouter<Arc<NetworkGraph>, Arc<LnCtlLogger>>;

pub(crate) fn init_invoice_payer<E: EventHandler>(
    channel_manager: Arc<ChannelManager>,
    network_graph: Arc<NetworkGraph>,
    scorer: Arc<Mutex<Scorer>>,
    event_handler: E,
    logger: Arc<LnCtlLogger>,
) -> Arc<InvoicePayer<E>> {
    let router = DefaultRouter::new(network_graph.clone(), logger.clone());
    Arc::new(InvoicePayer::new(
        channel_manager.clone(),
        router,
        scorer.clone(),
        logger.clone(),
        event_handler,
        payment::RetryAttempts(5),
    ))
}
