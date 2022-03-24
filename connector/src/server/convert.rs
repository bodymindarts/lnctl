use super::proto::*;
use crate::{gossip::GossipMessage, node_client::NodeType};

impl From<GossipMessage> for NodeEvent {
    fn from(msg: GossipMessage) -> Self {
        let ln_gossip = match msg {
            GossipMessage::NodeAnnouncement { node_id } => LnGossip {
                message: Some(ln_gossip::Message::NodeAnnouncement(NodeAnnouncement {
                    node_id: node_id.to_string(),
                })),
            },
        };
        NodeEvent {
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
