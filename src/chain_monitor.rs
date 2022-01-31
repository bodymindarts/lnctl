use crate::{bitcoind::BitcoindClient, logger::LnCtlLogger};
use lightning::chain::{chainmonitor, keysinterface::InMemorySigner, Filter};
use lightning_persister::FilesystemPersister;
use std::sync::Arc;

pub type ChainMonitor = chainmonitor::ChainMonitor<
    InMemorySigner,
    Arc<dyn Filter + Send + Sync>,
    Arc<BitcoindClient>,
    Arc<BitcoindClient>,
    Arc<LnCtlLogger>,
    Arc<FilesystemPersister>,
>;

pub fn init_chain_monitor(
    bitcoind_client: Arc<BitcoindClient>,
    logger: Arc<LnCtlLogger>,
    persistor: Arc<FilesystemPersister>,
) -> Arc<ChainMonitor> {
    Arc::new(chainmonitor::ChainMonitor::new(
        None,
        Arc::clone(&bitcoind_client),
        logger,
        bitcoind_client,
        persistor,
    ))
}
