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

pub async fn get_status(config: ClientConfig) -> anyhow::Result<()> {
    let mut client =
        LnCtlCoordinatorClient::connect(format!("http://{}:{}", config.addr, config.port)).await?;

    let request = tonic::Request::new(GetStatusRequest {});

    let response = client.get_status(request).await?;

    let peers = response.into_inner().connectors;
    println!("{}", serde_json::to_string_pretty(&peers).unwrap());

    Ok(())
}
