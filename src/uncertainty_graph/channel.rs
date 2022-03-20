use lightning::routing::network_graph::NodeId;

pub type ChannelId = u64;
pub struct UncertaintyChannel {
    pub node_one: NodeId,
    pub node_two: NodeId,
}
