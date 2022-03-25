pub mod proto {
    tonic::include_proto!("coordinator");
}

use tonic::{transport::Server, Request, Response, Status};

use crate::{config::ServerConfig, connector::Connectors};
use proto::{
    lnctl_coordinator_server::{LnctlCoordinator, LnctlCoordinatorServer},
    *,
};

struct CoordinatorServer {
    connectors: Connectors,
}

impl CoordinatorServer {
    pub fn new(connectors: Connectors) -> Self {
        Self { connectors }
    }
}

#[tonic::async_trait]
impl LnctlCoordinator for CoordinatorServer {
    async fn list_connectors(
        &self,
        _request: Request<ListConnectorsRequest>,
    ) -> Result<Response<ListConnectorsResponse>, Status> {
        let connectors = self.connectors.read().await;
        let ret = ListConnectorsResponse {
            connectors: connectors
                .iter()
                .map(|(id, con)| ConnectorInfo {
                    id: id.to_string(),
                    monitored_node_id: con.monitored_node_id.clone(),
                    r#type: con.r#type.clone(),
                })
                .collect(),
        };
        Ok(Response::new(ret))
    }
}

pub async fn run_server(config: ServerConfig, connectors: Connectors) -> anyhow::Result<()> {
    println!("Listening on port {}", config.port);
    Server::builder()
        .add_service(LnctlCoordinatorServer::new(CoordinatorServer::new(
            connectors,
        )))
        .serve(([0, 0, 0, 0], config.port).into())
        .await?;
    Ok(())
}
