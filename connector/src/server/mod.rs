mod convert;
mod proto {
    tonic::include_proto!("connector");
}

use bitcoin::secp256k1::PublicKey;
use futures::Stream;
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use crate::{
    config::{self, ServerConfig},
    node_client::NodeClient,
    update::ConnectorUpdate,
};
use proto::{
    lnctl_connector_server::{LnctlConnector, LnctlConnectorServer},
    *,
};

type ConnectorResponse<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<UpdateEvent, Status>> + Send>>;

struct ConnectorServer {
    uuid: Uuid,
    node_pubkey: PublicKey,
    node_client: Arc<RwLock<dyn NodeClient + Send + Sync + 'static>>,
    update_clients: Arc<RwLock<HashMap<Uuid, mpsc::Sender<UpdateEvent>>>>,
}

impl ConnectorServer {
    pub fn new(
        uuid: Uuid,
        node_pubkey: PublicKey,
        node_client: Arc<RwLock<dyn NodeClient + Send + Sync + 'static>>,
    ) -> Self {
        let update_clients = Arc::new(RwLock::new(HashMap::new()));
        Self {
            uuid,
            node_pubkey,
            update_clients,
            node_client,
        }
    }

    pub fn spawn_fanout_updates(
        mut incoming_updates: mpsc::Receiver<ConnectorUpdate>,
        clients: Arc<RwLock<HashMap<Uuid, mpsc::Sender<UpdateEvent>>>>,
    ) {
        tokio::spawn(async move {
            while let Some(item) = incoming_updates.recv().await {
                let event = UpdateEvent::from(item);
                let mut remove_clients = Vec::new();
                {
                    let clients = clients.read().await;
                    for (client_id, tx) in clients.iter() {
                        if let Err(_) = tx.send(UpdateEvent::clone(&event)).await {
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
            connector_id: self.uuid.to_string(),
            node_pubkey: self.node_pubkey.to_string(),
            r#type: proto::ConnectorType::from(client.node_type()) as i32,
        }))
    }

    type StreamUpdatesStream = ResponseStream;
    async fn stream_updates(
        &self,
        _request: Request<StreamUpdatesRequest>,
    ) -> ConnectorResponse<Self::StreamUpdatesStream> {
        let (tx, rx) = mpsc::channel(config::DEFAULT_CHANNEL_SIZE);
        let mut clients = self.update_clients.write().await;
        clients.insert(Uuid::new_v4(), tx);

        let output_stream = ReceiverStream::new(rx).map(|update| Ok(update));
        Ok(Response::new(
            Box::pin(output_stream) as Self::StreamUpdatesStream
        ))
    }
}

pub async fn run_server(
    config: ServerConfig,
    uuid: Uuid,
    node_pubkey: PublicKey,
    node_client: Arc<RwLock<dyn NodeClient + Send + Sync + 'static>>,
) -> anyhow::Result<()> {
    println!("Connector {} - monitoring {}", uuid, node_pubkey);
    println!("Listening on port {}", config.port);
    Server::builder()
        .add_service(LnctlConnectorServer::new(ConnectorServer::new(
            uuid,
            node_pubkey,
            node_client,
        )))
        .serve(([0, 0, 0, 0], config.port).into())
        .await?;
    Ok(())
}

impl From<ConnectorUpdate> for UpdateEvent {
    fn from(_: ConnectorUpdate) -> Self {
        UpdateEvent {}
    }
}
