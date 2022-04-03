use super::results;
use ::shared::proto::*;
use tonic::transport::channel::Channel;
pub type LnCtlGatewayClient = lnctl_gateway_client::LnctlGatewayClient<Channel>;

pub struct ClientConfig {
    pub addr: String,
    pub port: u16,
}

pub struct GatewayClient {
    config: ClientConfig,
}
impl GatewayClient {
    pub fn new(config: ClientConfig) -> GatewayClient {
        GatewayClient { config: config }
    }

    async fn connect(&self) -> anyhow::Result<LnCtlGatewayClient> {
        let client = LnCtlGatewayClient::connect(format!(
            "http://{}:{}",
            self.config.addr, self.config.port
        ))
        .await?;
        Ok(client)
    }

    pub async fn get_status(&self) -> anyhow::Result<()> {
        let mut client = self.connect().await?;

        let request = tonic::Request::new(gateway::GetStatusRequest {});

        let response = client.get_status(request).await?;

        let peers: Vec<_> = response
            .into_inner()
            .connectors
            .into_iter()
            .map(results::ConnectorInfo::from)
            .collect();

        println!("{}", serde_json::to_string_pretty(&peers).unwrap());

        Ok(())
    }

    pub async fn channel_history(&self, short_channel_id: u64) -> anyhow::Result<()> {
        let mut client = self.connect().await?;

        let request =
            tonic::Request::new(gateway::ListMonitoredChannelSnapshotsRequest { short_channel_id });

        let response = client.list_monitored_channel_snapshots(request).await?;

        let history = results::ChannelHistory::from(response.into_inner());

        println!("{}", serde_json::to_string_pretty(&history).unwrap());

        Ok(())
    }
}
