use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use crate::{config::ServerConfig, connector::Connectors, db::Db};
use ::shared::proto::{
    self,
    coordinator::*,
    lnctl_coordinator_server::{LnctlCoordinator, LnctlCoordinatorServer},
};

struct CoordinatorServer {
    id: Uuid,
    connectors: Connectors,
    db: Db,
}

impl CoordinatorServer {
    pub fn new(id: Uuid, connectors: Connectors, db: Db) -> Self {
        Self { id, connectors, db }
    }
}

#[tonic::async_trait]
impl LnctlCoordinator for CoordinatorServer {
    async fn get_status(
        &self,
        _request: Request<GetStatusRequest>,
    ) -> Result<Response<GetStatusResponse>, Status> {
        let connectors = self.connectors.read().await;
        let ret = GetStatusResponse {
            coordinator_id: self.id.to_string(),
            connectors: connectors
                .iter()
                .map(|(id, con)| ConnectorInfo {
                    id: id.to_string(),
                    monitored_node_id: con.monitored_node_id.to_string(),
                    r#type: con.r#type.clone(),
                })
                .collect(),
        };
        Ok(Response::new(ret))
    }

    async fn list_monitored_channel_snapshots(
        &self,
        request: Request<ListMonitoredChannelSnapshotsRequest>,
    ) -> Result<Response<ListMonitoredChannelSnapshotsResponse>, Status> {
        unimplemented!()
    }
}

pub(crate) async fn run_server(
    config: ServerConfig,
    id: Uuid,
    connectors: Connectors,
    db: Db,
) -> anyhow::Result<()> {
    println!("Listening on port {}", config.port);
    Server::builder()
        .add_service(LnctlCoordinatorServer::new(CoordinatorServer::new(
            id, connectors, db,
        )))
        .serve(([0, 0, 0, 0], config.port).into())
        .await?;
    Ok(())
}
