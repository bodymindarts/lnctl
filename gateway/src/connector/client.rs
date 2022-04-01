use anyhow::Context;
use tonic::transport::channel::Channel;

use crate::bus::*;
use ::shared::{
    primitives::*,
    proto::{self, connector::*},
};
pub type LnCtlConnectorClient = lnctl_connector_client::LnctlConnectorClient<Channel>;

pub(crate) struct ConnectorClient {
    pub connector_id: ConnectorId,
    pub monitored_node_id: MonitoredNodeId,
    pub address: String,
    pub r#type: String,
}

impl ConnectorClient {
    pub async fn connect(address: &str, bus: GatewayBus) -> anyhow::Result<Self> {
        let mut client = LnCtlConnectorClient::connect(format!("http://{}", address))
            .await
            .context("couldn't establish connection")?;
        let status = client.get_status(GetStatusRequest {}).await?.into_inner();
        let connector_id = status.connector_id.parse()?;
        let monitored_node_id = status.monitored_node_id.parse()?;
        Self::spawn_messages_stream(bus, client, connector_id, monitored_node_id);
        Ok(Self {
            connector_id,
            r#type: proto::ConnectorType::from_i32(status.r#type)
                .map(String::from)
                .unwrap_or("unknown".to_string()),
            address: address.to_string(),
            monitored_node_id,
        })
    }

    fn spawn_messages_stream(
        bus: GatewayBus,
        mut client: LnCtlConnectorClient,
        connector_id: ConnectorId,
        monitored_node_id: MonitoredNodeId,
    ) {
        tokio::spawn(async move {
            if let Ok(response) = client.stream_node_events(StreamNodeEventsRequest {}).await {
                let mut stream = response.into_inner();
                while let Ok(Some(node_event)) = stream.message().await {
                    if let Err(_) = bus
                        .dispatch(ConnectorMessage {
                            connector_id: connector_id.clone(),
                            monitored_node_id: monitored_node_id.clone(),
                            received_at: UnixTimestampSecs::now(),
                            node_event,
                        })
                        .await
                    {
                        eprintln!("Error forwarding ConnectionMessage");
                    }
                }
            }
        });
    }
}
