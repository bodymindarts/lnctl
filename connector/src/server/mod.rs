mod convert;
mod proto {
    tonic::include_proto!("connector");
}

use futures::Stream;
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use crate::{
    config::{self, ServerConfig},
    gossip::GossipMessage,
    node_client::NodeClient,
    primitives::*,
};
use proto::{
    lnctl_connector_server::{LnctlConnector, LnctlConnectorServer},
    *,
};

type ConnectorResponse<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<NodeEvent, Status>> + Send>>;

struct ConnectorServer {
    connector_id: ConnectorId,
    node_pubkey: MonitoredNodeId,
    node_client: Arc<RwLock<dyn NodeClient + Send + Sync + 'static>>,
    node_event_clients: Arc<RwLock<HashMap<Uuid, mpsc::Sender<NodeEvent>>>>,
}

impl ConnectorServer {
    pub fn new(
        connector_id: ConnectorId,
        node_pubkey: MonitoredNodeId,
        gossip_receiver: mpsc::Receiver<GossipMessage>,
        node_client: Arc<RwLock<dyn NodeClient + Send + Sync + 'static>>,
    ) -> Self {
        let node_event_clients = Arc::new(RwLock::new(HashMap::new()));
        Self::spawn_fanout_updates(
            connector_id,
            node_pubkey,
            gossip_receiver,
            Arc::clone(&node_event_clients),
        );
        Self {
            connector_id,
            node_pubkey,
            node_event_clients,
            node_client,
        }
    }

    pub fn spawn_fanout_updates(
        connector_id: ConnectorId,
        node_pubkey: MonitoredNodeId,
        mut gossip_receiver: mpsc::Receiver<GossipMessage>,
        clients: Arc<RwLock<HashMap<Uuid, mpsc::Sender<NodeEvent>>>>,
    ) {
        tokio::spawn(async move {
            while let Some(item) = gossip_receiver.recv().await {
                println!("Forwarding gossip message {:?}", item);
                let event = NodeEvent::from((connector_id, node_pubkey, item));
                let mut remove_clients = Vec::new();
                {
                    let clients = clients.read().await;
                    for (client_id, tx) in clients.iter() {
                        if let Err(_) = tx.send(NodeEvent::clone(&event)).await {
                            remove_clients.push(*client_id);
                        }
                    }
                }
                if remove_clients.len() > 0 {
                    let mut clients = clients.write().await;
                    for client_id in remove_clients {
                        clients.remove(&client_id);
                    }
                }
            }
        });
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
            node_pubkey: self.node_pubkey.to_string(),
            r#type: proto::ConnectorType::from(client.node_type()) as i32,
        }))
    }

    type StreamNodeEventsStream = ResponseStream;
    async fn stream_node_events(
        &self,
        _request: Request<StreamNodeEventsRequest>,
    ) -> ConnectorResponse<Self::StreamNodeEventsStream> {
        let (tx, rx) = mpsc::channel(config::DEFAULT_CHANNEL_SIZE);
        let mut clients = self.node_event_clients.write().await;
        clients.insert(Uuid::new_v4(), tx);

        let output_stream = ReceiverStream::new(rx).map(|update| Ok(update));
        Ok(Response::new(
            Box::pin(output_stream) as Self::StreamNodeEventsStream
        ))
    }
}

pub async fn run_server(
    config: ServerConfig,
    connector_id: ConnectorId,
    node_pubkey: MonitoredNodeId,
    receiver: mpsc::Receiver<GossipMessage>,
    node_client: Arc<RwLock<dyn NodeClient + Send + Sync + 'static>>,
) -> anyhow::Result<()> {
    println!("Connector {} - monitoring {}", connector_id, node_pubkey);
    println!("Listening on port {}", config.port);
    Server::builder()
        .add_service(LnctlConnectorServer::new(ConnectorServer::new(
            connector_id,
            node_pubkey,
            receiver,
            node_client,
        )))
        .serve(([0, 0, 0, 0], config.port).into())
        .await?;
    Ok(())
}
