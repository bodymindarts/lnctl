mod client;
mod file;
mod message;

use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::{mpsc, RwLock, RwLockReadGuard};
use uuid::Uuid;

pub use client::ConnectorClient;
pub use message::ConnectorMessage;
use shared::primitives::ConnectorId;

pub struct Connectors {
    inner: Arc<RwLock<HashMap<ConnectorId, ConnectorClient>>>,
}

impl Connectors {
    pub async fn new(
        connectors_file: PathBuf,
        sender: mpsc::Sender<ConnectorMessage>,
    ) -> anyhow::Result<Self> {
        let file_changes = file::watch(connectors_file).await?;
        let connectors = Arc::new(RwLock::new(HashMap::new()));
        Self::spawn_connect_from_list(Arc::clone(&connectors), sender, file_changes);
        Ok(Self { inner: connectors })
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, HashMap<ConnectorId, ConnectorClient>> {
        self.inner.read().await
    }

    fn spawn_connect_from_list(
        connectors: Arc<RwLock<HashMap<ConnectorId, ConnectorClient>>>,
        sender: mpsc::Sender<ConnectorMessage>,
        mut file_changes: mpsc::Receiver<Vec<String>>,
    ) {
        tokio::spawn(async move {
            while let Some(mut list) = file_changes.recv().await {
                println!("Updating connectors list");
                let mut existing_addresses = Vec::new();
                {
                    let mut connectors = connectors.write().await;
                    connectors.retain(|_, con| {
                        existing_addresses.push(con.address.clone());
                        list.contains(&con.address)
                    })
                }
                for address in list.drain(..) {
                    if !existing_addresses.contains(&address) {
                        println!("Connecting to {}", address);
                        match ConnectorClient::connect(&address, sender.clone()).await {
                            Ok(con) => {
                                println!(
                                    "Connection to connector {} established @ {}",
                                    con.connector_id, address
                                );
                                let mut connectors = connectors.write().await;
                                connectors.insert(con.connector_id, con);
                            }
                            Err(e) => println!("Failed to connect to {}: {}", address, e),
                        }
                    }
                }
            }
        });
    }
}
