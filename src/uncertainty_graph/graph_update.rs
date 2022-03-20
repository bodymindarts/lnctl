use super::channel::*;
use lightning::routing::network_graph::NodeId;

#[derive(Debug, Clone)]
pub enum GraphUpdate {
    UpdateNode {
        node_id: NodeId,
    },
    RemoveChannel {
        channel_id: ChannelId,
    },
    UpdateChannel {
        channel_id: ChannelId,
        node_a: NodeId,
        node_b: NodeId,
        total_capacity: Option<Satoshis>,
        a_to_b_info: Option<ChannelDirectionInfo>,
        b_to_a_info: Option<ChannelDirectionInfo>,
    },
}
