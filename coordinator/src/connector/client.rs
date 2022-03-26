mod proto {
    tonic::include_proto!("connector");
}

use anyhow::Context;
use tokio::sync::mpsc;
use uuid::Uuid;

use super::message::ConnectorMessage;
use proto::*;
use tonic::transport::channel::Channel;
pub type LnCtlConnectorClient = lnctl_connector_client::LnctlConnectorClient<Channel>;

pub struct ConnectorClient {
    inner: LnCtlConnectorClient,

    pub connector_id: Uuid,
    pub monitored_node_id: String,
    pub address: String,
    pub r#type: String,
}

impl ConnectorClient {
    pub async fn connect(
        address: &str,
        events: mpsc::Sender<ConnectorMessage>,
    ) -> anyhow::Result<Self> {
        let mut inner = LnCtlConnectorClient::connect(format!("http://{}", address))
            .await
            .context("couldn't establish connection")?;
        let status = inner.get_status(GetStatusRequest {}).await?.into_inner();
        Ok(Self {
            connector_id: status.connector_id.parse()?,
            r#type: proto::ConnectorType::from_i32(status.r#type)
                .map(String::from)
                .unwrap_or("unknown".to_string()),
            address: address.to_string(),
            monitored_node_id: status.monitored_node_id,
            inner,
        })
    }
}

mod convert {
    impl From<super::proto::ConnectorType> for String {
        fn from(t: super::proto::ConnectorType) -> Self {
            match t {
                super::proto::ConnectorType::Lnd => "lnd".to_string(),
            }
        }
    }
}
