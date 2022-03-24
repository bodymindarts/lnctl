use crate::primitives::MonitoredNodeId;
use bitcoin::secp256k1::PublicKey;

pub enum NodeType {
    Lnd,
}

pub struct NodeInfo {
    pub node_id: MonitoredNodeId,
    pub network: bitcoin::Network,
}

#[tonic::async_trait]
pub trait NodeClient {
    fn node_type(&self) -> NodeType;
    async fn node_info(&mut self) -> anyhow::Result<NodeInfo>;
    async fn connect_to_peer(
        &mut self,
        node_pubkey: PublicKey,
        node_addr: String,
    ) -> anyhow::Result<()>;
}
