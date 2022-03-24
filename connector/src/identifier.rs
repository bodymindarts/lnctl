use bitcoin::secp256k1::{PublicKey, SecretKey};
use uuid::Uuid;

pub struct ConnectorIdentifier {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub uuid: Uuid,
}
