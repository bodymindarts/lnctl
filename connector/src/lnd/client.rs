use anyhow::Context;
use bitcoin::secp256k1::PublicKey;
use std::str::FromStr;
use tonic_lnd::{rpc::*, Client as InnerClient};

use super::config::LndConnectorConfig;
use crate::{node_client::*, primitives::MonitoredNodeId};

pub struct LndClient {
    inner: InnerClient,
}

impl LndClient {
    pub async fn new(config: LndConnectorConfig) -> anyhow::Result<Self> {
        let inner = tonic_lnd::connect(
            format!("https://{}", config.admin_endpoint),
            config.cert_path,
            config.macaroon_path,
        )
        .await
        .context("Creating lnd client")?;
        Ok(Self { inner })
    }
}

#[tonic::async_trait]
impl NodeClient for LndClient {
    fn node_type(&self) -> NodeType {
        NodeType::Lnd
    }

    async fn node_pubkey(&mut self) -> anyhow::Result<MonitoredNodeId> {
        let response = self.inner.get_info(GetInfoRequest {}).await?;
        Ok(PublicKey::from_str(&response.into_inner().identity_pubkey)?.into())
    }

    async fn connect_to_peer(
        &mut self,
        node_pubkey: PublicKey,
        node_addr: String,
    ) -> anyhow::Result<()> {
        let request = ConnectPeerRequest {
            addr: Some(LightningAddress {
                pubkey: node_pubkey.to_string(),
                host: node_addr,
            }),
            perm: true,
            timeout: 30,
        };
        let _ = self.inner.connect_peer(request).await?;
        Ok(())
    }
}
