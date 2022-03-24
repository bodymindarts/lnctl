use super::proto::*;
use crate::{gossip::GossipMessage, node_client::NodeType};

impl From<GossipMessage> for NodeEvent {
    fn from(msg: GossipMessage) -> Self {
        let ln_gossip = match msg {
            GossipMessage::NodeAnnouncement { node_id } => LnGossip {
                message: Some(ln_gossip::Message::NodeAnnouncement(NodeAnnouncement {
                    node_id: node_id.to_string(),
                })),
            },
            GossipMessage::ChannelAnnouncement {
                short_channel_id,
                node_a_id,
                node_b_id,
            } => LnGossip {
                message: Some(ln_gossip::Message::ChannelAnnouncement(
                    ChannelAnnouncement {
                        short_channel_id,
                        node_a_id: node_a_id.to_string(),
                        node_b_id: node_b_id.to_string(),
                    },
                )),
            },
            GossipMessage::ChannelUpdate {
                short_channel_id,
                timestamp,
                cltv_expiry_delta,
                htlc_minimum_msat,
                htlc_maximum_msat,
                fee_base_msat,
                fee_proportional_millionths,
            } => LnGossip {
                message: Some(ln_gossip::Message::ChannelUpdate(ChannelUpdate {
                    short_channel_id,
                    timestamp,
                    cltv_expiry_delta: cltv_expiry_delta as u32,
                    htlc_minimum_msat: htlc_minimum_msat.into(),
                    htlc_maximum_msat: htlc_maximum_msat.map(u64::from),
                    fee_base_msat: fee_base_msat.into(),
                    fee_proportional_millionths,
                })),
            },
        };
        NodeEvent {
            event: Some(node_event::Event::Gossip(ln_gossip)),
        }
    }
}

impl From<NodeType> for ConnectorType {
    fn from(node_type: NodeType) -> Self {
        match node_type {
            NodeType::Lnd => ConnectorType::Lnd,
        }
    }
}
