use super::proto::*;
use crate::{gossip::GossipMessage, node_client::NodeType};
use bitcoin::secp256k1::PublicKey;
use uuid::Uuid;

impl From<(Uuid, PublicKey, GossipMessage)> for NodeEvent {
    fn from((uuid, node_pubkey, msg): (Uuid, PublicKey, GossipMessage)) -> Self {
        let ln_gossip = match msg {
            GossipMessage::NodeAnnouncement { pubkey } => LnGossip {
                message: Some(ln_gossip::Message::NodeAnnouncement(NodeAnnouncement {
                    announced_pubkey: pubkey.to_string(),
                })),
            },
        };
        NodeEvent {
            connector_id: uuid.to_string(),
            node_pubkey: node_pubkey.to_string(),
            event: Some(node_event::Event::Gossip(ln_gossip)),
        }
    }
}

impl From<NodeType> for ConnectorType {
    fn from(node_type: NodeType) -> Self {
        match node_type {
            NodeType::Lnd => ConnectorType::Lnd,
        }
    }
}
