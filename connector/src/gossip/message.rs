use bitcoin::secp256k1::PublicKey;

#[derive(Debug)]
pub enum GossipMessage {
    NodeAnnouncement { pubkey: PublicKey },
}
