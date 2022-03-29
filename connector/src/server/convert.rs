use super::proto;
use crate::{bus::*, db::flat, node_client::NodeType};
use shared::{bus::BusSubscriber, utils::hex_str};

impl From<LdkGossip> for proto::LnGossip {
    fn from(msg: LdkGossip) -> Self {
        use lightning::ln::msgs::*;
        match msg {
            LdkGossip::NodeAnnouncement {
                received_at,
                msg:
                    UnsignedNodeAnnouncement {
                        timestamp, node_id, ..
                    },
            } => proto::LnGossip {
                received_at: received_at.into(),
                message: Some(proto::ln_gossip::Message::NodeAnnouncement(
                    proto::NodeAnnouncement {
                        timestamp,
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
                            timestamp,
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

impl<'a> From<flat::GossipRecord<'a>> for Option<proto::LnGossip> {
    fn from(record: flat::GossipRecord) -> Self {
        let msg = match record.msg_type() {
            flat::Message::NodeAnnouncement => {
                let node_announcement = record.msg_as_node_announcement().unwrap();
                proto::ln_gossip::Message::NodeAnnouncement(proto::NodeAnnouncement {
                    timestamp: node_announcement.timestamp(),
                    node_id: hex_str(&node_announcement.node_id().unwrap().0),
                })
            }
            flat::Message::ChannelAnnouncement => {
                let channel_announcement = record.msg_as_channel_announcement().unwrap();
                proto::ln_gossip::Message::ChannelAnnouncement(proto::ChannelAnnouncement {
                    short_channel_id: channel_announcement.short_channel_id(),
                    node_a_id: hex_str(&channel_announcement.node_a_id().unwrap().0),
                    node_b_id: hex_str(&channel_announcement.node_b_id().unwrap().0),
                })
            }
            flat::Message::ChannelUpdate => {
                let channel_update = record.msg_as_channel_update().unwrap();
                let htlc_maximum_msat = channel_update.htlc_maximum_msat();
                proto::ln_gossip::Message::ChannelUpdate(proto::ChannelUpdate {
                    short_channel_id: channel_update.short_channel_id(),
                    timestamp: channel_update.timestamp(),
                    channel_enabled: channel_update.channel_enabled(),
                    channel_direction: proto::Direction::from(channel_update.direction()) as i32,
                    cltv_expiry_delta: channel_update.cltv_expiry_delta().into(),
                    htlc_minimum_msat: channel_update.htlc_minimum_msat(),
                    htlc_maximum_msat: if htlc_maximum_msat > 0 {
                        Some(htlc_maximum_msat)
                    } else {
                        None
                    },
                    fee_base_msat: channel_update.fee_base_msat(),
                    fee_proportional_millionths: channel_update.fee_proportional_millionths(),
                })
            }
            _ => return None,
        };

        Some(proto::LnGossip {
            received_at: record.received_at(),
            message: Some(msg),
        })
    }
}

impl From<flat::ChannelDirection> for proto::Direction {
    fn from(direction: flat::ChannelDirection) -> Self {
        match direction {
            flat::ChannelDirection::AToB => proto::Direction::AToB,
            flat::ChannelDirection::BToA => proto::Direction::BToA,
            _ => unimplemented!(),
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
