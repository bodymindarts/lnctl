use super::{bitcoind::BitcoindClient, logger::LnCtlLogger};
use lightning::{
    chain::{self, chainmonitor, keysinterface::InMemorySigner, Filter},
    ln::peer_handler::SimpleArcPeerManager,
};
use lightning_net_tokio::SocketDescriptor;
use lightning_persister::FilesystemPersister;
use std::sync::Arc;

type ChainMonitor = chainmonitor::ChainMonitor<
    InMemorySigner,
    Arc<dyn Filter + Send + Sync>,
    Arc<BitcoindClient>,
    Arc<BitcoindClient>,
    Arc<LnCtlLogger>,
    Arc<FilesystemPersister>,
>;

pub(crate) type LnPeers = SimpleArcPeerManager<
    SocketDescriptor,
    ChainMonitor,
    BitcoindClient,
    BitcoindClient,
    dyn chain::Access + Send + Sync,
    LnCtlLogger,
>;
