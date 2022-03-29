use bitcoin::secp256k1::PublicKey;

use crate::bus::ChannelState;
use shared::primitives::*;

pub enum NodeType {
    Lnd,
}

pub struct NodeInfo {
    pub node_id: MonitoredNodeId,
    pub network: bitcoin::Network,
}

#[tonic::async_trait]
pub(crate) trait NodeClient {
    fn node_type(&self) -> NodeType;
    async fn node_info(&mut self) -> anyhow::Result<NodeInfo>;
    async fn connect_to_peer(
        &mut self,
        node_pubkey: PublicKey,
        node_addr: String,
    ) -> anyhow::Result<()>;
    async fn list_channel_states(&mut self) -> anyhow::Result<Vec<ChannelState>>;
}
