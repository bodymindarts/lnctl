use super::proto::ConnectorType;
use crate::node_client::NodeType;

impl From<NodeType> for ConnectorType {
    fn from(node_type: NodeType) -> Self {
        match node_type {
            NodeType::Lnd => ConnectorType::Lnd,
        }
    }
}
