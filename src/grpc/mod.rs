use crate::{
    channel_manager::LnCtlChannelManager, hex_utils::hex_str, peers::LnCtlPeers,
    uncertainty_graph::LnGraph,
};
use lightning::{
    chain::{
        self,
        chaininterface::{BroadcasterInterface, FeeEstimator},
        keysinterface::{KeysInterface, Sign},
    },
    ln::{
        channelmanager::ChannelManager,
        msgs::{ChannelMessageHandler, RoutingMessageHandler},
        peer_handler::{CustomMessageHandler, PeerManager, SocketDescriptor},
    },
    routing::network_graph::ChannelInfo,
    util::logger::*,
};
use proto::{
    lnctl_server::{Lnctl, LnctlServer},
    *,
};
use std::{marker::Send, ops::Deref, sync::Arc};
use tonic::{transport::Server, Request, Response, Status};

pub mod proto {
    tonic::include_proto!("lnctl");
}

pub struct LnCtlGrpc<
    Descriptor: SocketDescriptor,
    Signer: Sign,
    M: Deref,
    T: Deref,
    K: Deref,
    F: Deref,
    CM: Deref,
    RM: Deref,
    CMH: Deref,
    L: Deref,
> where
    L::Target: Logger,
    M::Target: chain::Watch<Signer>,
    T::Target: BroadcasterInterface,
    K::Target: KeysInterface<Signer = Signer>,
    F::Target: FeeEstimator,
    CM::Target: ChannelMessageHandler,
    RM::Target: RoutingMessageHandler,
    CMH::Target: CustomMessageHandler,
{
    peer_manager: Arc<PeerManager<Descriptor, CM, RM, L, CMH>>,
    channel_manager: Arc<ChannelManager<Signer, M, T, K, F, L>>,
    network_graph: Arc<LnGraph>,
}

#[tonic::async_trait]
impl<
        Descriptor: SocketDescriptor,
        Signer: Sign,
        M: Deref,
        T: Deref,
        K: Deref,
        F: Deref,
        CM: Deref,
        RM: Deref,
        CMH: Deref,
        L: Deref,
    > Lnctl for LnCtlGrpc<Descriptor, Signer, M, T, K, F, CM, RM, CMH, L>
where
    L::Target: Logger,
    M::Target: chain::Watch<Signer>,
    T::Target: BroadcasterInterface,
    K::Target: KeysInterface<Signer = Signer>,
    F::Target: FeeEstimator,
    CM::Target: ChannelMessageHandler,
    RM::Target: RoutingMessageHandler,
    CMH::Target: CustomMessageHandler,
    L::Target: Logger + Send + Sync,
    Descriptor: Send + Sync + 'static,
    Signer: 'static + Send + Sync,
    M: 'static + Send + Sync,
    T: 'static + Send + Sync,
    K: 'static + Send + Sync,
    F: 'static + Send + Sync,
    CM: 'static + Send + Sync,
    RM: 'static + Send + Sync,
    CMH: 'static + Send + Sync,
    L: 'static + Send + Sync,
{
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
        let graph = self.network_graph.read_only();
        let channels = graph
            .channels()
            .values()
            .map(
                |ChannelInfo {
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
    network_graph: Arc<LnGraph>,
) -> anyhow::Result<()> {
    Server::builder()
        .add_service(LnctlServer::new(LnCtlGrpc {
            peer_manager,
            channel_manager,
            network_graph,
        }))
        .serve(([0, 0, 0, 0], port).into())
        .await?;
    Ok(())
}
