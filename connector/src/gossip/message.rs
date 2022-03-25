use crate::primitives::{MilliSatoshi, NodeId, UnixTimestampSecs};

#[derive(Debug, Clone, Copy)]
pub enum ChannelDirection {
    AToB,
    BToA,
}

#[derive(Debug)]
pub enum Message {
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
        update_counter: u32,
        channel_enabled: bool,
        direction: ChannelDirection,
        cltv_expiry_delta: u16,
        htlc_minimum_msat: MilliSatoshi,
        htlc_maximum_msat: Option<MilliSatoshi>,
        fee_base_msat: MilliSatoshi,
        fee_proportional_millionths: u32,
    },
}

#[derive(Debug)]
pub struct GossipMessage {
    pub received_at: UnixTimestampSecs,
    pub msg: Message,
}
