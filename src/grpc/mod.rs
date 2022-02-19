use crate::ln_peers::LnPeers;
use lightning::{
    ln::{
        msgs::{ChannelMessageHandler, RoutingMessageHandler},
        peer_handler::{CustomMessageHandler, PeerManager, SocketDescriptor},
    },
    util::logger::*,
};
use proto::{
    lnctl_server::{Lnctl, LnctlServer},
    ListPeersRequest, ListPeersResponse, Peer,
};
use std::{marker::Send, ops::Deref, sync::Arc};
use tonic::{transport::Server, Request, Response, Status};

pub mod proto {
    tonic::include_proto!("lnctl");
}

pub struct LnCtlGrpc<Descriptor: SocketDescriptor, L: Deref, CM: Deref, RM: Deref, CMH: Deref>
where
    L::Target: Logger,
    CM::Target: ChannelMessageHandler,
    RM::Target: RoutingMessageHandler,
    CMH::Target: CustomMessageHandler,
{
    peer_manager: Arc<PeerManager<Descriptor, CM, RM, L, CMH>>,
}

#[tonic::async_trait]
impl<Descriptor: SocketDescriptor, L: Deref, CM: Deref, RM: Deref, CMH: Deref> Lnctl
    for LnCtlGrpc<Descriptor, L, CM, RM, CMH>
where
    L::Target: Logger,
    CM::Target: ChannelMessageHandler,
    RM::Target: RoutingMessageHandler,
    CMH::Target: CustomMessageHandler,
    L::Target: Logger + Send + Sync,
    Descriptor: Send + Sync + 'static,
    CM: 'static + Send + Sync,
    RM: 'static + Send + Sync,
    CMH: 'static + Send + Sync,
    L: 'static + Send + Sync,
{
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
}

pub async fn start_server<
    Descriptor: SocketDescriptor,
    L: Deref,
    CM: Deref,
    RM: Deref,
    CMH: Deref,
>(
    port: u16,
    peer_manager: Arc<LnPeers>,
) -> anyhow::Result<()>
where
    L::Target: Logger,
    CM::Target: ChannelMessageHandler,
    RM::Target: RoutingMessageHandler,
    CMH::Target: CustomMessageHandler,
    L::Target: Logger + Send + Sync,
    Descriptor: Send + Sync + 'static,
    CM: 'static + Send + Sync,
    RM: 'static + Send + Sync,
    CMH: 'static + Send + Sync,
    L: 'static + Send + Sync,
{
    Server::builder()
        .add_service(LnctlServer::new(LnCtlGrpc { peer_manager }))
        .serve(([0, 0, 0, 0], port).into())
        .await?;
    Ok(())
}
