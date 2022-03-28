mod convert;
pub mod proto {
    tonic::include_proto!("connector");
}

use futures::Stream;
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use crate::{
    bus::*,
    config::{self, ServerConfig},
    db::Db,
    node_client::NodeClient,
};
use proto::{
    lnctl_connector_server::{LnctlConnector, LnctlConnectorServer},
    *,
};
use shared::primitives::*;

type ConnectorResponse<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<NodeEvent, Status>> + Send>>;

struct ConnectorServer {
    connector_id: ConnectorId,
    monitored_node_id: MonitoredNodeId,
    bus: ConnectorBus,
    node_client: Arc<RwLock<dyn NodeClient + Send + Sync + 'static>>,
    db: Db,
}

impl ConnectorServer {
    pub fn new(
        connector_id: ConnectorId,
        node_pubkey: MonitoredNodeId,
        bus: ConnectorBus,
        node_client: Arc<RwLock<dyn NodeClient + Send + Sync + 'static>>,
        db: Db,
    ) -> Self {
        let spawn_bus = bus.clone();
        tokio::spawn(async move {
            let mut stream = spawn_bus.subscribe::<proto::LnGossip>().await;
            while let Some(ln_gossip) = stream.next().await {
                if let Err(e) = spawn_bus
                    .dispatch(proto::NodeEvent {
                        event: Some(proto::node_event::Event::Gossip(ln_gossip)),
                    })
                    .await
                {
                    eprintln!("Could not dispatch: {:?}", e);
                }
            }
        });
        Self {
            connector_id,
            monitored_node_id: node_pubkey,
            bus,
            node_client,
            db,
        }
    }
}

#[tonic::async_trait]
impl LnctlConnector for ConnectorServer {
    async fn get_status(
        &self,
        _request: Request<GetStatusRequest>,
    ) -> Result<Response<GetStatusResponse>, Status> {
        let client = self.node_client.write().await;

        Ok(Response::new(GetStatusResponse {
            connector_id: self.connector_id.to_string(),
            monitored_node_id: self.monitored_node_id.to_string(),
            r#type: proto::ConnectorType::from(client.node_type()) as i32,
        }))
    }

    type StreamNodeEventsStream = ResponseStream;
    async fn stream_node_events(
        &self,
        _request: Request<StreamNodeEventsRequest>,
    ) -> ConnectorResponse<Self::StreamNodeEventsStream> {
        let stream = self.db.load_gossip();
        let output_stream = stream
            .map(|gossip| proto::NodeEvent {
                event: Some(proto::node_event::Event::Gossip(gossip)),
            })
            .chain(self.bus.subscribe::<proto::NodeEvent>().await)
            .map(|event| Ok(event));
        Ok(Response::new(
            Box::pin(output_stream) as Self::StreamNodeEventsStream
        ))
    }
}

pub(crate) async fn run_server(
    config: ServerConfig,
    connector_id: ConnectorId,
    node_pubkey: MonitoredNodeId,
    bus: ConnectorBus,
    node_client: Arc<RwLock<dyn NodeClient + Send + Sync + 'static>>,
    db: Db,
) -> anyhow::Result<()> {
    println!("Connector {} - monitoring {}", connector_id, node_pubkey);
    println!("Listening on port {}", config.port);
    Server::builder()
        .add_service(LnctlConnectorServer::new(ConnectorServer::new(
            connector_id,
            node_pubkey,
            bus,
            node_client,
            db,
        )))
        .serve(([0, 0, 0, 0], config.port).into())
        .await?;
    Ok(())
}
