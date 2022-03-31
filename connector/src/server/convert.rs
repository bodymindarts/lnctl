use crate::{bus::*, db::flat, node_client::NodeType};
use ::shared::proto;
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

impl<'a> TryFrom<flat::gossip::GossipRecord<'a>> for proto::LnGossip {
    type Error = ();

    fn try_from(record: flat::gossip::GossipRecord) -> Result<Self, Self::Error> {
        let msg = match record.msg_type() {
            flat::gossip::Message::NodeAnnouncement => {
                let node_announcement = record.msg_as_node_announcement().unwrap();
                proto::ln_gossip::Message::NodeAnnouncement(proto::NodeAnnouncement {
                    timestamp: node_announcement.timestamp(),
                    node_id: hex_str(&node_announcement.node_id().unwrap().0),
                })
            }
            flat::gossip::Message::ChannelAnnouncement => {
                let channel_announcement = record.msg_as_channel_announcement().unwrap();
                proto::ln_gossip::Message::ChannelAnnouncement(proto::ChannelAnnouncement {
                    short_channel_id: channel_announcement.short_channel_id(),
                    node_a_id: hex_str(&channel_announcement.node_a_id().unwrap().0),
                    node_b_id: hex_str(&channel_announcement.node_b_id().unwrap().0),
                })
            }
            flat::gossip::Message::ChannelUpdate => {
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
            _ => return Err(()),
        };

        Ok(proto::LnGossip {
            received_at: record.received_at(),
            message: Some(msg),
        })
    }
}

impl<'a> TryFrom<flat::channels::ChannelScrape<'a>> for proto::MonitoredChannelUpdate {
    type Error = ();

    fn try_from(channel_scrape: flat::channels::ChannelScrape<'a>) -> Result<Self, Self::Error> {
        if let Some(channel_state) = channel_scrape.state() {
            Ok(proto::MonitoredChannelUpdate {
                timestamp: channel_scrape.scrape_timestamp(),
                channel_state: Some(proto::ChannelState {
                    short_channel_id: channel_state.short_channel_id(),
                    local_node_id: hex_str(&channel_state.local_node_id().unwrap().0),
                    remote_node_id: hex_str(&channel_state.remote_node_id().unwrap().0),
                    active: channel_state.active(),
                    private: channel_state.private(),
                    capacity: channel_state.capacity(),
                    local_balance: channel_state.local_balance(),
                    remote_balance: channel_state.remote_balance(),
                    unsettled_balance: channel_state.unsettled_balance(),
                    local_channel_settings: Some(proto::ChannelSettings {
                        chan_reserve_sat: channel_state
                            .local_channel_settings()
                            .unwrap()
                            .chan_reserve_sat(),
                        htlc_minimum_msat: channel_state
                            .local_channel_settings()
                            .unwrap()
                            .htlc_minimum_msat(),
                    }),
                    remote_channel_settings: Some(proto::ChannelSettings {
                        chan_reserve_sat: channel_state
                            .remote_channel_settings()
                            .unwrap()
                            .chan_reserve_sat(),
                        htlc_minimum_msat: channel_state
                            .remote_channel_settings()
                            .unwrap()
                            .htlc_minimum_msat(),
                    }),
                }),
            })
        } else {
            Err(())
        }
    }
}

impl From<MonitoredChannelUpdate> for proto::MonitoredChannelUpdate {
    fn from(
        MonitoredChannelUpdate {
            scrape:
                ChannelScrape {
                    scraped_at,
                    state:
                        ChannelState {
                            short_channel_id,
                            local_node_id,
                            remote_node_id,
                            active,
                            private,
                            capacity,
                            local_balance,
                            remote_balance,
                            unsettled_balance,
                            local_channel_settings:
                                ChannelSettings {
                                    chan_reserve_sat: local_chan_reserve_sat,
                                    htlc_minimum_msat: local_htlm_minimum_msat,
                                },
                            remote_channel_settings:
                                ChannelSettings {
                                    chan_reserve_sat: remote_chan_reserve_sat,
                                    htlc_minimum_msat: remote_htlm_minimum_msat,
                                },
                        },
                },
        }: MonitoredChannelUpdate,
    ) -> Self {
        proto::MonitoredChannelUpdate {
            timestamp: scraped_at.into(),
            channel_state: Some(proto::ChannelState {
                short_channel_id,
                local_node_id: local_node_id.to_string(),
                remote_node_id: remote_node_id.to_string(),
                active,
                private,
                capacity: capacity.into(),
                local_balance: local_balance.into(),
                remote_balance: remote_balance.into(),
                unsettled_balance: unsettled_balance.into(),
                local_channel_settings: Some(proto::ChannelSettings {
                    chan_reserve_sat: local_chan_reserve_sat.into(),
                    htlc_minimum_msat: local_htlm_minimum_msat.into(),
                }),
                remote_channel_settings: Some(proto::ChannelSettings {
                    chan_reserve_sat: remote_chan_reserve_sat.into(),
                    htlc_minimum_msat: remote_htlm_minimum_msat.into(),
                }),
            }),
        }
    }
}
impl From<flat::gossip::ChannelDirection> for proto::Direction {
    fn from(direction: flat::gossip::ChannelDirection) -> Self {
        match direction {
            flat::gossip::ChannelDirection::AToB => proto::Direction::AToB,
            flat::gossip::ChannelDirection::BToA => proto::Direction::BToA,
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

impl BusSubscriber<BusMessage> for proto::MonitoredChannelUpdate {
    fn filter(msg: &BusMessage) -> bool {
        if let BusMessage::ChannelUpdate(_) = msg {
            true
        } else {
            false
        }
    }

    fn convert(msg: BusMessage) -> Option<Self> {
        if let BusMessage::ChannelUpdate(event) = msg {
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
