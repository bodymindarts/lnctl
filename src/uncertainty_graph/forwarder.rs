use super::{channel::*, graph_update::*};
use lightning::routing::network_graph::{ChannelInfo, DirectionalChannelInfo, NodeId, NodeInfo};
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

    pub fn remove_channel(&self, channel_id: u64) {
        self.send_msg(GraphUpdate::RemoveChannel { channel_id })
    }

    pub fn update_channel(&self, channel_id: u64, info: &ChannelInfo) {
        self.send_msg(GraphUpdate::UpdateChannel {
            channel_id,
            node_a: info.node_one,
            node_b: info.node_two,
            total_capacity: info.capacity_sats.map(Satoshis::from),
            a_to_b_info: info.one_to_two.as_ref().map(
                |DirectionalChannelInfo {
                     fees,
                     enabled,
                     htlc_maximum_msat,
                     htlc_minimum_msat,
                     ..
                 }| ChannelDirectionInfo {
                    enabled: *enabled,
                    fees: *fees,
                    send_min: MilliSatoshis::from(htlc_minimum_msat),
                    send_max: htlc_maximum_msat.as_ref().map(MilliSatoshis::from),
                },
            ),
            b_to_a_info: info.two_to_one.as_ref().map(
                |DirectionalChannelInfo {
                     fees,
                     enabled,
                     htlc_maximum_msat,
                     htlc_minimum_msat,
                     ..
                 }| ChannelDirectionInfo {
                    enabled: *enabled,
                    fees: *fees,
                    send_min: MilliSatoshis::from(htlc_minimum_msat),
                    send_max: htlc_maximum_msat.as_ref().map(MilliSatoshis::from),
                },
            ),
        })
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
