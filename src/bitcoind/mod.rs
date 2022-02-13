mod client;
mod convert;

use super::config::BitcoindConfig;
use anyhow::*;
pub use client::*;
use std::{fs, sync::Arc};

pub async fn init_bitcoind_client(
    BitcoindConfig {
        rpc_user,
        rpc_password_file,
        rpc_host,
        rpc_port,
        network,
    }: BitcoindConfig,
) -> Result<Arc<BitcoindClient>, anyhow::Error> {
    let mut rpc_password =
        fs::read_to_string(rpc_password_file).context("Could not read bitcoind_password_file")?;
    if rpc_password.ends_with('\n') {
        rpc_password.pop();
    }

    let client = Arc::new(
        BitcoindClient::new(
            network,
            rpc_host,
            rpc_port,
            rpc_user,
            rpc_password,
            tokio::runtime::Handle::current(),
        )
        .await
        .context("Could not create bitcoind client")?,
    );
    let bitcoind_chain = client.get_blockchain_info().await?.chain;
    if bitcoind_chain
        != match network {
            bitcoin::Network::Bitcoin => "main",
            bitcoin::Network::Testnet => "test",
            bitcoin::Network::Regtest => "regtest",
            bitcoin::Network::Signet => "signet",
        }
    {
        Err(anyhow!(
            "Bitcoind is on the wrong network. Expected {} but got {}",
            bitcoind_chain,
            network
        ))
    } else {
        Ok(client)
    }
}
