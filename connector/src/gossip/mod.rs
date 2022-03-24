mod forwarder;
mod logger;

use lightning::ln::peer_handler::{
    ErroringMessageHandler, IgnoringMessageHandler, MessageHandler, PeerManager,
};
use lightning_net_tokio::SocketDescriptor;
use std::sync::Arc;

use forwarder::RoutingMessageForwarder;
use logger::LnLogger;

pub(crate) type LnPeers = PeerManager<
    SocketDescriptor,
    Arc<ErroringMessageHandler>,
    Arc<RoutingMessageForwarder>,
    Arc<LnLogger>,
    Arc<IgnoringMessageHandler>,
>;

pub struct Gossip {}

impl Gossip {
    pub fn listen() -> anyhow::Result<Self> {
        let msg_handler = MessageHandler {
            chan_handler: ErroringMessageHandler::new(),
            route_handler: forwarder::RoutingMessageForwarder::new(),
        };
        // let peers = peers::LnPeers::new(msg_handler,
        //     );
        Ok(Self {})
    }
}
