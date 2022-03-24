use super::proto::*;
use crate::{gossip::GossipMessage, node_client::NodeType, primitives::*};

impl From<(ConnectorId, MonitoredNodeId, GossipMessage)> for NodeEvent {
    fn from(
        (connector_id, monitored_node_id, msg): (ConnectorId, MonitoredNodeId, GossipMessage),
    ) -> Self {
        let ln_gossip = match msg {
            GossipMessage::NodeAnnouncement { node_id } => LnGossip {
                message: Some(ln_gossip::Message::NodeAnnouncement(NodeAnnouncement {
                    node_id: node_id.to_string(),
                })),
            },
        };
        NodeEvent {
            connector_id: connector_id.to_string(),
            monitored_node_id: monitored_node_id.to_string(),
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
