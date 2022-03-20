use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    RwLock, RwLockReadGuard,
};

use super::{graph::UncertaintyGraph, graph_update::GraphUpdate};

pub struct GraphPool {
    handles: Vec<Arc<RwLock<UncertaintyGraph>>>,
}

impl GraphPool {
    pub(super) fn new(n_graphs: u8, incoming: Receiver<GraphUpdate>) -> Self {
        let mut senders = Vec::new();
        let mut handles = Vec::new();
        for _ in 0..n_graphs {
            let (sender, receiver) = mpsc::channel(10000);
            senders.push(sender);
            handles.push(UncertaintyGraph::new(receiver));
        }
        fan_out(incoming, senders);

        Self { handles }
    }

    pub async fn read_graph(&self) -> RwLockReadGuard<'_, UncertaintyGraph> {
        self.handles[0].read().await
    }
}

fn fan_out(mut incoming: Receiver<GraphUpdate>, senders: Vec<Sender<GraphUpdate>>) {
    tokio::spawn(async move {
        loop {
            if let Some(msg) = incoming.recv().await {
                println!("Forwarding msg to graphs {:?}", msg);
                for sender in &senders {
                    if let Err(_) = sender.send(msg.clone()).await {
                        eprintln!("Warinng: couldn't fan out msg")
                    }
                }
            } else {
                break;
            }
        }
    });
}
