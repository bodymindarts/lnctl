use lightning::routing::network_graph::NodeId;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc::Receiver, RwLock};

use super::{
    channel::UncertaintyChannel, channel::*, graph_update::GraphUpdate, node::UncertaintyNode,
};

pub struct UncertaintyGraph {
    nodes: HashMap<NodeId, UncertaintyNode>,
    channels: HashMap<ChannelId, UncertaintyChannel>,
}

impl UncertaintyGraph {
    pub(super) fn new(receiver: Receiver<GraphUpdate>) -> Arc<RwLock<Self>> {
        let ret = Arc::new(RwLock::new(Self {
            nodes: HashMap::new(),
            channels: HashMap::new(),
        }));
        play_updates(receiver, Arc::clone(&ret));
        ret
    }

    pub fn nodes(&self) -> &HashMap<NodeId, UncertaintyNode> {
        &self.nodes
    }

    pub fn channels(&self) -> &HashMap<ChannelId, UncertaintyChannel> {
        &self.channels
    }

    fn handle_update(&mut self, update: GraphUpdate) {
        match update {
            GraphUpdate::UpdateNode { node_id } => {
                let _ = self.nodes.insert(node_id, UncertaintyNode {});
            }
        }
    }
}

fn play_updates(mut receiver: Receiver<GraphUpdate>, handle: Arc<RwLock<UncertaintyGraph>>) {
    tokio::spawn(async move {
        loop {
            if let Some(msg) = receiver.recv().await {
                let mut graph = handle.write().await;
                graph.handle_update(msg);
                while let Ok(msg) = receiver.try_recv() {
                    graph.handle_update(msg);
                }
            } else {
                break;
            }
        }
    });
}
