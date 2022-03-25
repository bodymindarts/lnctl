use zerocopy::*;

use crate::gossip::{GossipMessage, Message};

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
pub struct GossipMessageKey {
    short_channel_id: U64<BigEndian>,
    update_counter: U32<BigEndian>,
    pubkey: [u8; 33],
}

impl From<&GossipMessage> for GossipMessageKey {
    fn from(msg: &GossipMessage) -> Self {
        match msg.msg {
            Message::NodeAnnouncement { node_id } => Self {
                short_channel_id: U64::new(0),
                update_counter: U32::new(0),
                pubkey: node_id.serialize(),
            },
            Message::ChannelAnnouncement {
                short_channel_id, ..
            } => Self {
                short_channel_id: U64::new(short_channel_id),
                update_counter: U32::new(0),
                pubkey: [0; 33],
            },
            Message::ChannelUpdate {
                short_channel_id,
                update_counter,
                ..
            } => Self {
                short_channel_id: U64::new(short_channel_id),
                update_counter: U32::new(update_counter),
                pubkey: [0; 33],
            },
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::gossip::{ChannelDirection, GossipMessage, Message};
    use crate::primitives::{MilliSatoshi, NodeId, UnixTimestampSecs};

    #[test]
    fn key_order() {
        let node_id = "026e5bab59b8c8a977dc7a2b86368a82324382c71949b43619003c2d4b6ae1d4dd"
            .parse::<NodeId>()
            .unwrap();
        let short_channel_id = 123456789;
        let msg = GossipMessage {
            received_at: UnixTimestampSecs::now(),
            msg: Message::NodeAnnouncement { node_id },
        };
        let node_announce_key = GossipMessageKey::from(&msg);
        let msg = GossipMessage {
            received_at: UnixTimestampSecs::now(),
            msg: Message::ChannelAnnouncement {
                short_channel_id,
                node_a_id: node_id,
                node_b_id: node_id,
            },
        };
        let channel_announce_key = GossipMessageKey::from(&msg);
        let msg = GossipMessage {
            received_at: UnixTimestampSecs::now(),
            msg: Message::ChannelUpdate {
                short_channel_id,
                update_counter: 1,
                channel_enabled: true,
                cltv_expiry_delta: 10,
                direction: ChannelDirection::AToB,
                htlc_minimum_msat: MilliSatoshi::from(10000_u64),
                htlc_maximum_msat: None,
                fee_base_msat: MilliSatoshi::from(0_u64),
                fee_proportional_millionths: 0,
            },
        };
        let first_update = GossipMessageKey::from(&msg);
        let msg = GossipMessage {
            received_at: UnixTimestampSecs::now(),
            msg: Message::ChannelUpdate {
                short_channel_id,
                update_counter: 123456789,
                channel_enabled: true,
                cltv_expiry_delta: 10,
                direction: ChannelDirection::AToB,
                htlc_minimum_msat: MilliSatoshi::from(10000_u64),
                htlc_maximum_msat: None,
                fee_base_msat: MilliSatoshi::from(0_u64),
                fee_proportional_millionths: 0,
            },
        };
        let second_update = GossipMessageKey::from(&msg);
        assert_eq!(
            node_announce_key
                .as_bytes()
                .cmp(channel_announce_key.as_bytes()),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            channel_announce_key.as_bytes().cmp(first_update.as_bytes()),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            first_update.as_bytes().cmp(second_update.as_bytes()),
            std::cmp::Ordering::Less
        );
    }
}
