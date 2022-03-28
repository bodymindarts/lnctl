use super::proto;
use crate::{bus::*, node_client::NodeType};
use shared::bus::BusSubscriber;

impl From<LdkGossip> for proto::LnGossip {
    fn from(msg: LdkGossip) -> Self {
        use lightning::ln::msgs::*;
        match msg {
            LdkGossip::NodeAnnouncement {
                received_at,
                msg: UnsignedNodeAnnouncement { node_id, .. },
            } => proto::LnGossip {
                received_at: received_at.into(),
                message: Some(proto::ln_gossip::Message::NodeAnnouncement(
                    proto::NodeAnnouncement {
                        node_id: node_id.to_string(),
                    },
                )),
            },
            LdkGossip::ChannelAnnouncement {
                received_at,
                msg:
                    UnsignedChannelAnnouncement {
                        short_channel_id,
                        node_id_1,
                        node_id_2,
                        ..
                    },
            } => proto::LnGossip {
                received_at: received_at.into(),
                message: Some(proto::ln_gossip::Message::ChannelAnnouncement(
                    proto::ChannelAnnouncement {
                        short_channel_id,
                        node_a_id: node_id_1.to_string(),
                        node_b_id: node_id_2.to_string(),
                    },
                )),
            },
            LdkGossip::ChannelUpdate {
                received_at,
                msg:
                    UnsignedChannelUpdate {
                        short_channel_id,
                        timestamp,
                        cltv_expiry_delta,
                        htlc_minimum_msat,
                        htlc_maximum_msat,
                        fee_base_msat,
                        fee_proportional_millionths,
                        flags,
                        ..
                    },
            } => {
                let channel_enabled = flags & (1 << 1) != (1 << 1);
                let direction = if flags & 1 == 1 {
                    proto::Direction::BToA
                } else {
                    proto::Direction::AToB
                };

                proto::LnGossip {
                    received_at: received_at.into(),
                    message: Some(proto::ln_gossip::Message::ChannelUpdate(
                        proto::ChannelUpdate {
                            short_channel_id,
                            update_counter: timestamp,
                            channel_direction: direction as i32,
                            channel_enabled,
                            cltv_expiry_delta: cltv_expiry_delta as u32,
                            htlc_minimum_msat: htlc_minimum_msat.into(),
                            htlc_maximum_msat: if let OptionalField::Present(msats) =
                                htlc_maximum_msat
                            {
                                Some(msats.into())
                            } else {
                                None
                            },
                            fee_base_msat: u64::from(fee_base_msat) as u32,
                            fee_proportional_millionths,
                        },
                    )),
                }
            }
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

impl BusSubscriber<BusMessage> for proto::LnGossip {
    fn filter(msg: &BusMessage) -> bool {
        if let BusMessage::LdkGossip(_) = msg {
            true
        } else {
            false
        }
    }

    fn convert(msg: BusMessage) -> Option<Self> {
        if let BusMessage::LdkGossip(event) = msg {
            Some(event.into())
        } else {
            None
        }
    }
}

impl BusSubscriber<BusMessage> for proto::NodeEvent {
    fn filter(msg: &BusMessage) -> bool {
        if let BusMessage::NodeEvent(_) = msg {
            true
        } else {
            false
        }
    }

    fn convert(msg: BusMessage) -> Option<Self> {
        if let BusMessage::NodeEvent(event) = msg {
            Some(event)
        } else {
            None
        }
    }
}
