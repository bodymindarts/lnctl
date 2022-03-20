use super::{
    bitcoind::BitcoindClient, chain_monitor::ChainMonitor, channel_manager::LnCtlChannelManager,
    logger::LnCtlLogger,
};
use crate::message_forwarder::MessageForwarder;
use bitcoin::secp256k1::PublicKey;
use lightning::{
    chain::keysinterface::{KeysInterface, KeysManager},
    ln::channelmanager::SimpleArcChannelManager,
    ln::peer_handler::{IgnoringMessageHandler, MessageHandler, PeerManager},
};
use lightning_net_tokio::SocketDescriptor;
use rand::{self, Rng};
use std::{net::SocketAddr, sync::Arc, time::Duration};

pub(crate) type LnCtlPeers = PeerManager<
    SocketDescriptor,
    Arc<SimpleArcChannelManager<ChainMonitor, BitcoindClient, BitcoindClient, LnCtlLogger>>,
    Arc<MessageForwarder>,
    Arc<LnCtlLogger>,
    Arc<IgnoringMessageHandler>,
>;

pub fn init_peer_manager(
    listening_port: u16,
    channel_manager: Arc<LnCtlChannelManager>,
    network_gossip: Arc<MessageForwarder>,
    keys_manager: Arc<KeysManager>,
    logger: Arc<LnCtlLogger>,
) -> Arc<LnCtlPeers> {
    let mut ephemeral_bytes = [0; 32];
    rand::thread_rng().fill_bytes(&mut ephemeral_bytes);
    let lightning_msg_handler = MessageHandler {
        chan_handler: channel_manager.clone(),
        route_handler: network_gossip.clone(),
    };
    let peer_manager = Arc::new(LnCtlPeers::new(
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
    peer_manager: Arc<LnCtlPeers>,
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

pub(crate) async fn connect_peer_if_necessary(
    pubkey: PublicKey,
    peer_addr: SocketAddr,
    peer_manager: Arc<LnCtlPeers>,
) -> Result<(), ()> {
    for node_pubkey in peer_manager.get_peer_node_ids() {
        if node_pubkey == pubkey {
            return Ok(());
        }
    }
    let res = do_connect_peer(pubkey, peer_addr, peer_manager).await;
    if res.is_err() {
        println!("ERROR: failed to connect to peer");
    }
    res
}
