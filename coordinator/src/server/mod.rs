mod proto {
    tonic::include_proto!("coordinator");
}

use tonic::{transport::Server, Request, Response, Status};

use crate::config::{self, ServerConfig};
use proto::{
    lnctl_coordinator_server::{LnctlCoordinator, LnctlCoordinatorServer},
    *,
};

struct CoordinatorServer {}

impl CoordinatorServer {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl LnctlCoordinator for CoordinatorServer {
    async fn list_connectors(
        &self,
        _request: Request<ListConnectorsRequest>,
    ) -> Result<Response<ListConnectorsResponse>, Status> {
        Ok(Response::new(ListConnectorsResponse { connectors: vec![] }))
    }
}

pub async fn run_server(config: ServerConfig) -> anyhow::Result<()> {
    Server::builder()
        .add_service(LnctlCoordinatorServer::new(CoordinatorServer::new()))
        .serve(([0, 0, 0, 0], config.port).into())
        .await?;
    Ok(())
}
