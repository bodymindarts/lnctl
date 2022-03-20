use lightning::routing::network_graph::NodeId;

pub enum GraphUpdate {
    UpdateNode { node_id: NodeId },
}
