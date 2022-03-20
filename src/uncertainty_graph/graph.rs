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

    fn add_node(&mut self, node_id: NodeId) {
        self.nodes.entry(node_id).or_insert(UncertaintyNode {});
    }

    fn handle_update(&mut self, update: GraphUpdate) {
        match update {
            GraphUpdate::UpdateNodeFromGossip { node_id } => self.add_node(node_id),
            GraphUpdate::RemoveChannelFromGossip { channel_id } => {
                let _ = self.channels.remove(&channel_id);
            }
            GraphUpdate::UpdateChannelFromGossip {
                channel_id,
                total_capacity,
                node_a,
                node_b,
                a_to_b_info,
                b_to_a_info,
            } => {
                self.add_node(node_a);
                self.add_node(node_b);
                let _ = self.channels.insert(
                    channel_id,
                    UncertaintyChannel {
                        total_capacity,
                        node_a,
                        node_b,
                        a_to_b_info,
                        b_to_a_info,
                    },
                );
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
