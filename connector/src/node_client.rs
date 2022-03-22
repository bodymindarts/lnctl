use bitcoin::secp256k1::PublicKey;

pub enum NodeType {
    Lnd,
}

#[tonic::async_trait]
pub trait NodeClient {
    fn node_type(&self) -> NodeType;
    async fn node_pubkey(&mut self) -> anyhow::Result<PublicKey>;
}
