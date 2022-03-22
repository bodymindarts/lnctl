mod proto {
    tonic::include_proto!("coordinator");
}

use proto::*;
use tonic::transport::channel::Channel;
pub type LnCtlCoordinatorClient = lnctl_coordinator_client::LnctlCoordinatorClient<Channel>;

pub struct ClientConfig {
    pub addr: String,
    pub port: u16,
}

pub async fn list_connectors(config: ClientConfig) -> anyhow::Result<()> {
    let mut client =
        LnCtlCoordinatorClient::connect(format!("http://{}:{}", config.addr, config.port)).await?;

    let request = tonic::Request::new(ListConnectorsRequest {});

    let response = client.list_connectors(request).await?;

    let peers = response.into_inner().connectors;
    println!("{}", serde_json::to_string_pretty(&peers).unwrap());

    Ok(())
}
