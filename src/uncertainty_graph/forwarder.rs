use super::graph_update::GraphUpdate;
use lightning::routing::network_graph::{NodeId, NodeInfo};
use tokio::sync::mpsc::Sender;

pub struct UncertaintyGraphMsgForwarder {
    channel: Sender<GraphUpdate>,
}

impl UncertaintyGraphMsgForwarder {
    pub fn new(channel: Sender<GraphUpdate>) -> Self {
        Self { channel }
    }

    pub fn update_node(&self, node_id: NodeId, _: &NodeInfo) {
        self.send_msg(GraphUpdate::UpdateNode { node_id });
    }

    fn send_msg(&self, msg: GraphUpdate) {
        let sender = self.channel.clone();
        tokio::spawn(async move {
            if let Err(e) = sender.send(msg).await {
                eprintln!("Warning: couldn't forward mesage: {:?}", e);
            }
        });
    }
}
