mod forwarder;
mod logger;
mod message;

use bitcoin::secp256k1::SecretKey;
use lightning::ln::peer_handler::{
    ErroringMessageHandler, IgnoringMessageHandler, MessageHandler, PeerManager,
};
use lightning_net_tokio::SocketDescriptor;
use rand::Rng;
use std::sync::Arc;
use tokio::sync::mpsc;

use forwarder::RoutingMessageForwarder;
use logger::LnLogger;
pub use message::GossipMessage;

pub(crate) type LnPeers = PeerManager<
    SocketDescriptor,
    Arc<ErroringMessageHandler>,
    Arc<RoutingMessageForwarder>,
    Arc<LnLogger>,
    Arc<IgnoringMessageHandler>,
>;

pub struct Gossip {}

impl Gossip {
    pub fn listen(listen_port: u16, connector_secret: SecretKey) -> mpsc::Receiver<GossipMessage> {
        let (send, receive) = mpsc::channel(50);
        let msg_handler = MessageHandler {
            chan_handler: Arc::new(ErroringMessageHandler::new()),
            route_handler: Arc::new(forwarder::RoutingMessageForwarder::new(send)),
        };
        let mut ephemeral_bytes = [0; 32];
        rand::thread_rng().fill_bytes(&mut ephemeral_bytes);
        let peers = LnPeers::new(
            msg_handler,
            connector_secret,
            &ephemeral_bytes,
            Arc::new(LnLogger::new()),
            Arc::new(IgnoringMessageHandler {}),
        );
        Self::spawn_peer_listener(listen_port, Arc::new(peers));
        receive
    }

    fn spawn_peer_listener(listen_port: u16, peer_manager: Arc<LnPeers>) {
        tokio::spawn(async move {
            println!("Listening for gossip on port {}", listen_port);
            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", listen_port))
                .await
                .expect(
                    "Failed to bind to listen port - is something else already listening on it?",
                );
            loop {
                let peer_mgr = Arc::clone(&peer_manager);
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
    }
}