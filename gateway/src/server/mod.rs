mod convert;

use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use crate::{config::ServerConfig, connector::Connectors, db::Db};
use ::shared::proto::{
    self,
    gateway::*,
    lnctl_gateway_server::{LnctlGateway, LnctlGatewayServer},
};

struct GatewayServer {
    id: Uuid,
    connectors: Connectors,
    db: Db,
}

impl GatewayServer {
    pub fn new(id: Uuid, connectors: Connectors, db: Db) -> Self {
        Self { id, connectors, db }
    }
}

#[tonic::async_trait]
impl LnctlGateway for GatewayServer {
    async fn get_status(
        &self,
        _request: Request<GetStatusRequest>,
    ) -> Result<Response<GetStatusResponse>, Status> {
        let connectors = self.connectors.read().await;
        let ret = GetStatusResponse {
            gateway_id: self.id.to_string(),
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
        let request = request.into_inner();
        let snapshots = self
            .db
            .list_monitored_channel_snapshots(request.short_channel_id)?;
        Ok(Response::new(ListMonitoredChannelSnapshotsResponse {
            snapshots,
        }))
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
        .add_service(LnctlGatewayServer::new(GatewayServer::new(
            id, connectors, db,
        )))
        .serve(([0, 0, 0, 0], config.port).into())
        .await?;
    Ok(())
}
