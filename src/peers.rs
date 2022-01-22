pub(crate) type PeerManager = SimpleArcPeerManager<
    SocketDescriptor,
    ChainMonitor,
    BitcoindClient,
    BitcoindClient,
    dyn chain::Access + Send + Sync,
    FilesystemLogger,
>;

pub(crate) fn connect_peer_manager(bitcoind_conf: &BitcoindConfig) {}
