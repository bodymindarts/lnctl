mod convert;

use anyhow::{anyhow, Context};
use bitcoin::{blockdata::block::Block, hash_types::BlockHash, hashes::hex::ToHex};
use lightning_block_sync::{
    http::{HttpEndpoint, JsonResponse},
    rpc::RpcClient,
};
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;

use convert::*;
use shared::primitives::Satoshi;

pub struct BitcoindClient {
    network: bitcoin::Network,
    bitcoind_rpc_client: Arc<RwLock<RpcClient>>,
    host: String,
    port: u16,
    rpc_user: String,
    rpc_password: String,
}

impl BitcoindClient {
    pub async fn new(
        network: bitcoin::Network,
        host: String,
        port: u16,
        rpc_user: String,
        rpc_password: String,
    ) -> std::io::Result<Self> {
        let http_endpoint = HttpEndpoint::for_host(host.clone()).with_port(port);
        let rpc_credentials =
            base64::encode(format!("{}:{}", rpc_user.clone(), rpc_password.clone()));
        let mut bitcoind_rpc_client = RpcClient::new(&rpc_credentials, http_endpoint)?;
        let _dummy = bitcoind_rpc_client
            .call_method::<BlockchainInfo>("getblockchaininfo", &[])
            .await
            .map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::PermissionDenied,
				"Failed to make initial call to bitcoind - please check your RPC user/password and access settings")
            })?;
        let client = Self {
            network,
            bitcoind_rpc_client: Arc::new(RwLock::new(bitcoind_rpc_client)),
            host,
            port,
            rpc_user,
            rpc_password,
        };
        Ok(client)
    }

    pub async fn lookup_channel_balance(&self, short_channel_id: u64) -> anyhow::Result<Satoshi> {
        let rpc_client = Arc::clone(&self.bitcoind_rpc_client);
        let block_height = block_from_scid(short_channel_id);
        let tx_index = tx_index_from_scid(short_channel_id);
        let vout = vout_from_scid(short_channel_id);
        let mut rpc = rpc_client.write().await;
        let BlockHashW(block_hash): BlockHashW = {
            let block_height = serde_json::json!(block_height);
            rpc.call_method("getblockheader", &[block_height]).await?
        };
        let mut block: Block = {
            let header_hash = serde_json::json!(block_hash.to_hex());
            let verbosity = serde_json::json!(0 as u16);
            rpc.call_method("getblock", &[header_hash, verbosity])
                .await?
        };
        let mut tx = block.txdata.remove(tx_index as usize);
        let out = tx.output.remove(vout as usize);
        Ok(Satoshi::from(out.value))
    }
}

const MAX_SCID_BLOCK: u64 = 0x00ffffff;
const MAX_SCID_TX_INDEX: u64 = 0x00ffffff;
const MAX_SCID_VOUT_INDEX: u64 = 0xffff;

fn block_from_scid(short_channel_id: u64) -> u32 {
    return (short_channel_id >> 40) as u32;
}

fn tx_index_from_scid(short_channel_id: u64) -> u32 {
    return ((short_channel_id >> 16) & MAX_SCID_TX_INDEX) as u32;
}

fn vout_from_scid(short_channel_id: u64) -> u16 {
    return ((short_channel_id) & MAX_SCID_VOUT_INDEX) as u16;
}
