use crate::{config::Config, grpc::proto::*};
use tonic::transport::channel::Channel;
pub type LnCtlClient = lnctl_client::LnctlClient<Channel>;

pub async fn list_peers(config: Config) -> anyhow::Result<()> {
    let mut client = LnCtlClient::connect(format!("http://localhost:{}", config.grpc_port)).await?;

    let request = tonic::Request::new(ListPeersRequest {});

    let response = client.list_peers(request).await?;

    let peers = response.into_inner().peers;
    println!("{}", serde_json::to_string_pretty(&peers).unwrap());

    Ok(())
}

pub async fn get_node_status(config: Config) -> anyhow::Result<()> {
    let mut client = LnCtlClient::connect(format!("http://localhost:{}", config.grpc_port)).await?;

    let request = tonic::Request::new(GetNodeStatusRequest {});

    let response = client.get_node_status(request).await?;

    println!(
        "{}",
        serde_json::to_string_pretty(&response.into_inner()).unwrap()
    );

    Ok(())
}

pub async fn get_network_graph(config: Config) -> anyhow::Result<()> {
    let mut client = LnCtlClient::connect(format!("http://localhost:{}", config.grpc_port)).await?;

    let request = tonic::Request::new(GetNetworkGraphRequest {});

    let response = client.get_network_graph(request).await?;

    println!(
        "{}",
        serde_json::to_string_pretty(&response.into_inner()).unwrap()
    );

    Ok(())
}
