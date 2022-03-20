use super::{channel_manager::LnCtlChannelManager, logger::LnCtlLogger};
use lightning::{
    routing::{network_graph::NetworkGraph, scoring::Scorer},
    util::events::EventHandler,
};
use lightning_invoice::{payment, utils::DefaultRouter};
use std::sync::{Arc, Mutex};

pub(crate) type InvoicePayer<E> = payment::InvoicePayer<
    Arc<LnCtlChannelManager>,
    Router,
    Arc<Mutex<Scorer>>,
    Arc<LnCtlLogger>,
    E,
>;
type Router = DefaultRouter<Arc<NetworkGraph>, Arc<LnCtlLogger>>;

pub(crate) fn init_invoice_payer<E: EventHandler>(
    channel_manager: Arc<LnCtlChannelManager>,
    network_graph: Arc<NetworkGraph>,
    scorer: Arc<Mutex<Scorer>>,
    event_handler: E,
    logger: Arc<LnCtlLogger>,
) -> Arc<InvoicePayer<E>> {
    let router = DefaultRouter::new(network_graph, logger.clone());
    Arc::new(InvoicePayer::new(
        channel_manager.clone(),
        router,
        scorer,
        logger,
        event_handler,
        payment::RetryAttempts(5),
    ))
}