use crate::primitives::NodeId;

#[derive(Debug)]
pub enum GossipMessage {
    NodeAnnouncement { node_id: NodeId },
}
