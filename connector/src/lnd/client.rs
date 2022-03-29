use anyhow::Context;
use bitcoin::secp256k1::PublicKey;
use std::str::FromStr;
use tonic_lnd::{rpc::*, Client as InnerClient};

use super::config::LndConnectorConfig;
use crate::{
    bus::{ChannelSettings, ChannelState},
    node_client::{self, NodeClient, NodeType},
};
use shared::primitives::*;

pub(crate) struct LndClient {
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

    async fn node_info(&mut self) -> anyhow::Result<node_client::NodeInfo> {
        let response = self.inner.get_info(GetInfoRequest {}).await?;
        let GetInfoResponse {
            identity_pubkey,
            chains,
            ..
        } = response.into_inner();
        let network = match chains.first() {
            Some(chain) => match chain.network.as_ref() {
                "mainnet" => bitcoin::Network::Bitcoin,
                "testnet" => bitcoin::Network::Testnet,
                "regtest" => bitcoin::Network::Regtest,
                _ => bitcoin::Network::Bitcoin,
            },
            None => bitcoin::Network::Bitcoin,
        };

        Ok(node_client::NodeInfo {
            node_id: MonitoredNodeId::from_str(&identity_pubkey)?,
            network,
        })
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

    async fn list_channel_states(&mut self) -> anyhow::Result<Vec<ChannelState>> {
        let node_client::NodeInfo { node_id, .. } = self.node_info().await?;
        let request = ListChannelsRequest {
            active_only: false,
            inactive_only: false,
            public_only: false,
            private_only: false,
            peer: Vec::new(),
        };
        let response = self.inner.list_channels(request).await?;
        let mut states = Vec::new();
        for channel in response.into_inner().channels {
            let Channel {
                chan_id,
                remote_pubkey,
                active,
                capacity,
                local_balance,
                remote_balance,
                unsettled_balance,
                private,
                local_constraints,
                remote_constraints,
                ..
            } = channel;
            if let (
                Some(ChannelConstraints {
                    chan_reserve_sat: local_reserve_sat,
                    min_htlc_msat: local_min_htlc_msat,
                    ..
                }),
                Some(ChannelConstraints {
                    chan_reserve_sat: remote_reserve_sat,
                    min_htlc_msat: remote_min_htlc_msat,
                    ..
                }),
            ) = (local_constraints, remote_constraints)
            {
                states.push(ChannelState {
                    short_channel_id: chan_id,
                    local_node_id: node_id,
                    remote_node_id: remote_pubkey.parse()?,
                    active,
                    private,
                    capacity: capacity.into(),
                    local_balance: local_balance.into(),
                    remote_balance: remote_balance.into(),
                    unsettled_balance: unsettled_balance.into(),
                    local_channel_settings: ChannelSettings {
                        chan_reserve_sat: local_reserve_sat.into(),
                        min_htlc_msat: local_min_htlc_msat.into(),
                    },
                    remote_channel_settings: ChannelSettings {
                        chan_reserve_sat: remote_reserve_sat.into(),
                        min_htlc_msat: remote_min_htlc_msat.into(),
                    },
                });
            }
        }
        Ok(states)
    }
}
