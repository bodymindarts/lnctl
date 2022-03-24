use bitcoin::secp256k1::PublicKey;

pub enum NodeType {
    Lnd,
}

#[tonic::async_trait]
pub trait NodeClient {
    fn node_type(&self) -> NodeType;
    async fn node_pubkey(&mut self) -> anyhow::Result<PublicKey>;
    async fn connect_to_peer(
        &mut self,
        node_pubkey: PublicKey,
        node_addr: String,
    ) -> anyhow::Result<()>;
}
