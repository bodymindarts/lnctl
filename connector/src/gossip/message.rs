use crate::primitives::{MilliSatoshi, NodeId};

#[derive(Debug)]
pub enum GossipMessage {
    NodeAnnouncement {
        node_id: NodeId,
    },
    ChannelAnnouncement {
        short_channel_id: u64,
        node_a_id: NodeId,
        node_b_id: NodeId,
    },
    ChannelUpdate {
        short_channel_id: u64,
        timestamp: u32,
        cltv_expiry_delta: u16,
        htlc_minimum_msat: MilliSatoshi,
        htlc_maximum_msat: Option<MilliSatoshi>,
        fee_base_msat: MilliSatoshi,
        fee_proportional_millionths: u32,
    },
}
