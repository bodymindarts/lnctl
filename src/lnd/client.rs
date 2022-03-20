use std::path::PathBuf;
use tonic::transport::channel::Channel;

use super::macaroon::*;

pub mod proto {
    tonic::include_proto!("lnrpc");
}

pub type Lnd = proto::lightning_client::LightningClient<Channel>;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LndConfig {
    admin_endpoint: String,
    macaroon_path: PathBuf,
}

pub struct LndClient {
    inner: Lnd,
    macaroon: MacaroonData,
}

impl LndClient {
    pub async fn new(config: LndConfig) -> anyhow::Result<LndClient> {
        let inner = Lnd::connect(config.admin_endpoint).await?;
        let macaroon = MacaroonData::from_file_path(config.macaroon_path)?;
        Ok(LndClient { inner, macaroon })
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
