use zerocopy::*;

use crate::bus::{ChannelScrape, LdkGossip};

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
pub struct GossipMessageKey {
    short_channel_id: U64<BigEndian>,
    update_counter: U32<BigEndian>,
    pubkey: [u8; 33],
}

impl From<&LdkGossip> for GossipMessageKey {
    fn from(msg: &LdkGossip) -> Self {
        use lightning::ln::msgs::*;
        match msg {
            LdkGossip::NodeAnnouncement {
                msg: UnsignedNodeAnnouncement { node_id, .. },
                ..
            } => Self {
                short_channel_id: U64::new(0),
                update_counter: U32::new(0),
                pubkey: node_id.serialize(),
            },
            LdkGossip::ChannelAnnouncement {
                msg:
                    UnsignedChannelAnnouncement {
                        short_channel_id, ..
                    },
                ..
            } => Self {
                short_channel_id: U64::new(*short_channel_id),
                update_counter: U32::new(0),
                pubkey: [0; 33],
            },
            LdkGossip::ChannelUpdate {
                msg:
                    UnsignedChannelUpdate {
                        short_channel_id,
                        timestamp,
                        ..
                    },
                ..
            } => Self {
                short_channel_id: U64::new(*short_channel_id),
                update_counter: U32::new(*timestamp),
                pubkey: [0; 33],
            },
        }
    }
}

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
pub struct ChannelStateKey {
    short_channel_id: U64<BigEndian>,
}
impl From<&ChannelScrape> for ChannelStateKey {
    fn from(scrape: &ChannelScrape) -> Self {
        Self {
            short_channel_id: U64::new(scrape.state.short_channel_id),
        }
    }
}
