use crate::{
    node::{channel_manager::LnCtlChannelManager, hex_utils::hex_str, peers::LnCtlPeers},
    uncertainty_graph::{GraphPool, UncertaintyChannel},
};
use proto::{
    lnctl_server::{Lnctl, LnctlServer},
    *,
};
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

pub mod proto {
    tonic::include_proto!("lnctl");
}

pub struct LnCtlGrpc {
    peer_manager: Arc<LnCtlPeers>,
    channel_manager: Arc<LnCtlChannelManager>,
    graph_pool: GraphPool,
}

#[tonic::async_trait]
impl Lnctl for LnCtlGrpc {
    async fn get_node_status(
        &self,
        _request: Request<GetNodeStatusRequest>,
    ) -> Result<Response<GetNodeStatusResponse>, Status> {
        let pubkey = self.channel_manager.get_our_node_id();
        let response = GetNodeStatusResponse {
            id: pubkey.to_string(),
        };
        Ok(Response::new(response))
    }

    async fn list_peers(
        &self,
        _request: Request<ListPeersRequest>,
    ) -> Result<Response<ListPeersResponse>, Status> {
        let peers = self
            .peer_manager
            .get_peer_node_ids()
            .iter()
            .map(|pubkey| Peer {
                pubkey: pubkey.to_string(),
            })
            .collect();
        Ok(Response::new(ListPeersResponse { peers }))
    }

    async fn get_network_graph(
        &self,
        _request: Request<GetNetworkGraphRequest>,
    ) -> Result<Response<GetNetworkGraphResponse>, Status> {
        let graph = self.graph_pool.read_graph().await;
        let channels = graph
            .channels()
            .values()
            .map(
                |UncertaintyChannel {
                     node_one, node_two, ..
                 }| Channel {
                    node_one: hex_str(node_one.as_slice()),
                    node_two: hex_str(node_two.as_slice()),
                },
            )
            .collect();
        let nodes = graph
            .nodes()
            .keys()
            .map(|node_id| Node {
                node_id: hex_str(node_id.as_slice()),
            })
            .collect();
        Ok(Response::new(GetNetworkGraphResponse { channels, nodes }))
    }
}

pub async fn start_server(
    port: u16,
    peer_manager: Arc<LnCtlPeers>,
    channel_manager: Arc<LnCtlChannelManager>,
    graph_pool: GraphPool,
) -> anyhow::Result<()> {
    Server::builder()
        .add_service(LnctlServer::new(LnCtlGrpc {
            peer_manager,
            channel_manager,
            graph_pool,
        }))
        .serve(([0, 0, 0, 0], port).into())
        .await?;
    Ok(())
}
