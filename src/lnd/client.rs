use anyhow::Context;
use std::path::PathBuf;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

use super::macaroon::*;

pub mod proto {
    tonic::include_proto!("lnrpc");
}

pub type Lnd = proto::lightning_client::LightningClient<Channel>;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LndConfig {
    admin_endpoint: String,
    macaroon_path: PathBuf,
    tls_cert_path: PathBuf,
}

pub(super) struct LndClient {
    inner: Lnd,
    macaroon: MacaroonData,
}

impl LndClient {
    pub async fn new(config: LndConfig) -> anyhow::Result<LndClient> {
        let macaroon = MacaroonData::from_file_path(config.macaroon_path)?;
        let data: Vec<u8> =
            std::fs::read(config.tls_cert_path).context("Couldn't read lnd cert")?;
        let cert = Certificate::from_pem(data);
        // let certs = rustls::pemfile::certs(
        //     std::fs::read(config.tls_cert_path).context("Couldn't read lnd cert")?,
        // );
        let client_config = rustls::ClientConfig::new();
        let channel = Channel::builder(format!("https://{}", config.admin_endpoint).parse()?)
            .tls_config(ClientTlsConfig::new().rustls_client_config(client_config))?
            .connect()
            .await?;
        Ok(LndClient {
            inner: Lnd::new(channel),
            macaroon,
        })
    }

    pub async fn list_channels(&mut self) -> anyhow::Result<Vec<proto::Channel>> {
        let mut request = tonic::Request::new(proto::ListChannelsRequest {
            active_only: false,
            inactive_only: false,
            private_only: false,
            public_only: false,
            peer: Vec::new(),
        });
        self.macaroon.add_to_metadata(request.metadata_mut());
        let res = self.inner.list_channels(request).await?;
        Ok(res.into_inner().channels)
    }

    pub async fn subscribe_channel_events(
        &mut self,
    ) -> anyhow::Result<tonic::Streaming<proto::ChannelEventUpdate>> {
        let mut request = tonic::Request::new(proto::ChannelEventSubscription {});
        self.macaroon.add_to_metadata(request.metadata_mut());
        let res = self.inner.subscribe_channel_events(request).await?;
        Ok(res.into_inner())
    }
}
