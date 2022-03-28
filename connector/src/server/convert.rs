use super::proto;
use crate::{bus::*, gossip::*, node_client::NodeType};

impl From<LdkGossip> for proto::NodeEvent {
    fn from(msg: LdkGossip) -> Self {
        use lightning::ln::msgs::*;
        let ln_gossip = match msg {
            LdkGossip::NodeAnnouncement {
                received_at,
                msg: UnsignedNodeAnnouncement { node_id ,..},
            } => proto::LnGossip {
                received_at: received_at.into(),
                message: Some(proto::ln_gossip::Message::NodeAnnouncement(proto::NodeAnnouncement {
                    node_id: node_id.to_string(),
                })),
            },
            _ => unimplemented!()
            // Message::ChannelAnnouncement {
            //     short_channel_id,
            //     node_a_id,
            //     node_b_id,
            // } => LnGossip {
            //     received_at: msg.received_at.into(),
            //     message: Some(ln_gossip::Message::ChannelAnnouncement(
            //         ChannelAnnouncement {
            //             short_channel_id,
            //             node_a_id: node_a_id.to_string(),
            //             node_b_id: node_b_id.to_string(),
            //         },
            //     )),
            // },
            // Message::ChannelUpdate {
            //     short_channel_id,
            //     update_counter,
            //     direction,
            //     channel_enabled,
            //     cltv_expiry_delta,
            //     htlc_minimum_msat,
            //     htlc_maximum_msat,
            //     fee_base_msat,
            //     fee_proportional_millionths,
            // } => LnGossip {
            //     received_at: msg.received_at.into(),
            //     message: Some(ln_gossip::Message::ChannelUpdate(ChannelUpdate {
            //         short_channel_id,
            //         update_counter,
            //         channel_direction: Direction::from(direction) as i32,
            //         channel_enabled,
            //         cltv_expiry_delta: cltv_expiry_delta as u32,
            //         htlc_minimum_msat: htlc_minimum_msat.into(),
            //         htlc_maximum_msat: htlc_maximum_msat.map(u64::from),
            //         fee_base_msat: u64::from(fee_base_msat) as u32,
            //         fee_proportional_millionths,
            //     })),
            // },
        };
        proto::NodeEvent {
            event: Some(proto::node_event::Event::Gossip(ln_gossip)),
        }
    }
}

impl From<NodeType> for proto::ConnectorType {
    fn from(node_type: NodeType) -> Self {
        match node_type {
            NodeType::Lnd => proto::ConnectorType::Lnd,
        }
    }
}
