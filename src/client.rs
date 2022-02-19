use crate::{
    config::Config,
    grpc::proto::{lnctl_client, ListPeersRequest},
};
use tonic::transport::channel::Channel;
pub type LnCtlClient = lnctl_client::LnctlClient<Channel>;

pub async fn list_peers(config: Config) -> anyhow::Result<()> {
    let mut client = LnCtlClient::connect(format!("http://localhost:{}", config.grpc_port)).await?;

    let request = tonic::Request::new(ListPeersRequest {});

    let response = client.list_peers(request).await?;

    let peers = response.into_inner().peers;
    println!("Peers:");
    if peers.len() == 0 {
        println!("  None");
    }
    peers.into_iter().for_each(|peer| {
        println!("{}", peer.pubkey);
    });

    Ok(())
}
