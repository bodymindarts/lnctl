mod proto {
    tonic::include_proto!("connector");
}

use anyhow::Context;
use uuid::Uuid;

use proto::*;
use tonic::transport::channel::Channel;
pub type LnCtlConnectorClient = lnctl_connector_client::LnctlConnectorClient<Channel>;

pub struct ConnectorClient {
    inner: LnCtlConnectorClient,

    pub id: Uuid,
    pub node_pubkey: String,
    pub address: String,
    pub r#type: String,
}
impl ConnectorClient {
    pub async fn connect(address: &str) -> anyhow::Result<Self> {
        let mut inner = LnCtlConnectorClient::connect(format!("http://{}", address))
            .await
            .context("couldn't establish connection")?;
        let status = inner.get_status(GetStatusRequest {}).await?.into_inner();
        Ok(Self {
            id: status.connector_id.parse()?,
            r#type: proto::ConnectorType::from_i32(status.r#type)
                .map(String::from)
                .unwrap_or("unknown".to_string()),
            address: address.to_string(),
            node_pubkey: status.node_pubkey,
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
