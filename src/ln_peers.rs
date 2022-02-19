use super::{
    bitcoind::BitcoindClient, channel_manager::ChannelManager, logger::LnCtlLogger,
    uncertainty_graph::ArcNetGraphMsgHandler,
};
use bitcoin::secp256k1::PublicKey;
use lightning::{
    chain::{
        self, chainmonitor,
        keysinterface::{InMemorySigner, KeysInterface, KeysManager},
        Filter,
    },
    ln::peer_handler::{IgnoringMessageHandler, MessageHandler, SimpleArcPeerManager},
};
use lightning_net_tokio::SocketDescriptor;
use lightning_persister::FilesystemPersister;
use rand::{self, Rng};
use std::{net::SocketAddr, sync::Arc, time::Duration};

pub type ChainMonitor = chainmonitor::ChainMonitor<
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

pub fn init_peer_manager(
    listening_port: u16,
    channel_manager: Arc<ChannelManager>,
    network_gossip: ArcNetGraphMsgHandler,
    keys_manager: Arc<KeysManager>,
    logger: Arc<LnCtlLogger>,
) -> Arc<LnPeers> {
    let mut ephemeral_bytes = [0; 32];
    rand::thread_rng().fill_bytes(&mut ephemeral_bytes);
    let lightning_msg_handler = MessageHandler {
        chan_handler: channel_manager.clone(),
        route_handler: network_gossip.clone(),
    };
    let peer_manager = Arc::new(LnPeers::new(
        lightning_msg_handler,
        keys_manager.get_node_secret(),
        &ephemeral_bytes,
        logger,
        Arc::new(IgnoringMessageHandler {}),
    ));

    let peer_manager_connection_handler = Arc::clone(&peer_manager);
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", listening_port))
            .await
            .expect("Failed to bind to listen port - is something else already listening on it?");
        loop {
            let peer_mgr = Arc::clone(&peer_manager_connection_handler);
            let tcp_stream = listener.accept().await.unwrap().0;
            tokio::spawn(async move {
                lightning_net_tokio::setup_inbound(
                    Arc::clone(&peer_mgr),
                    tcp_stream.into_std().unwrap(),
                )
                .await;
            });
        }
    });
    peer_manager
}

pub(crate) async fn do_connect_peer(
    pubkey: PublicKey,
    peer_addr: SocketAddr,
    peer_manager: Arc<LnPeers>,
) -> Result<(), ()> {
    match lightning_net_tokio::connect_outbound(Arc::clone(&peer_manager), pubkey, peer_addr).await
    {
        Some(connection_closed_future) => {
            let mut connection_closed_future = Box::pin(connection_closed_future);
            loop {
                match futures::poll!(&mut connection_closed_future) {
                    std::task::Poll::Ready(_) => {
                        return Err(());
                    }
                    std::task::Poll::Pending => {}
                }
                // Avoid blocking the tokio context by sleeping a bit
                match peer_manager
                    .get_peer_node_ids()
                    .iter()
                    .find(|id| **id == pubkey)
                {
                    Some(_) => return Ok(()),
                    None => tokio::time::sleep(Duration::from_millis(10)).await,
                }
            }
        }
        None => Err(()),
    }
}
