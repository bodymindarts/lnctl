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
            .iter()
            .map(
                |(
                    short_channel_id,
                    UncertaintyChannel {
                        node_a,
                        node_b,
                        total_capacity,
                        ..
                    },
                )| Channel {
                    short_channel_id: *short_channel_id,
                    node_a: hex_str(node_a.as_slice()),
                    node_b: hex_str(node_b.as_slice()),
                    total_capacity: total_capacity.as_ref().map(u64::from),
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
