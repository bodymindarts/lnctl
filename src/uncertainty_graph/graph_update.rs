use lightning::routing::network_graph::NodeId;

#[derive(Debug, Clone)]
pub enum GraphUpdate {
    UpdateNode { node_id: NodeId },
}
