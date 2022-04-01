use anyhow::{anyhow, Context};
use bitcoin::{hash_types::BlockHash, hashes::hex::FromHex};
use lightning_block_sync::http::JsonResponse;
use std::str::FromStr;

pub struct BlockchainInfo {
    pub latest_height: usize,
    pub latest_blockhash: BlockHash,
    pub chain: String,
}
impl TryInto<BlockchainInfo> for JsonResponse {
    type Error = std::io::Error;
    fn try_into(self) -> std::io::Result<BlockchainInfo> {
        Ok(BlockchainInfo {
            latest_height: self.0["blocks"].as_u64().unwrap() as usize,
            latest_blockhash: BlockHash::from_hex(self.0["bestblockhash"].as_str().unwrap())
                .unwrap(),
            chain: self.0["chain"].as_str().unwrap().to_string(),
        })
    }
}

pub(super) struct BlockHashW(pub BlockHash);
impl TryFrom<JsonResponse> for BlockHashW {
    type Error = std::io::Error;

    fn try_from(JsonResponse(value): JsonResponse) -> Result<BlockHashW, Self::Error> {
        use serde_json::Value;
        match value {
            Value::String(hash) => Ok(Self(
                BlockHash::from_str(&hash)
                    .context("Couldn't deserialize block hash")
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                anyhow!("json return value not a string"),
            )),
        }
    }
}
