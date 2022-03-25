use super::flat;
use crate::{
    gossip::{ChannelDirection, GossipMessage, Message},
    server::proto,
};

#[repr(transparent)]
pub struct FinishedBytes<'a>(&'a [u8]);
impl<'a> std::ops::Deref for FinishedBytes<'a> {
    type Target = &'a [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<(&'a mut flatbuffers::FlatBufferBuilder<'_>, &GossipMessage)> for FinishedBytes<'a> {
    fn from((builder, msg): (&'a mut flatbuffers::FlatBufferBuilder<'_>, &GossipMessage)) -> Self {
        builder.reset();
        let msg = match msg.msg {
            Message::NodeAnnouncement { node_id } => {
                let pubkey = flat::PubKey(node_id.serialize());
                let node_announcement = flat::NodeAnnouncement::create(
                    builder,
                    &flat::NodeAnnouncementArgs {
                        node_id: Some(&pubkey),
                    },
                );
                flat::GossipRecord::create(
                    builder,
                    &flat::GossipRecordArgs {
                        received_at: msg.received_at.into(),
                        msg_type: flat::Message::NodeAnnouncement,
                        msg: Some(node_announcement.as_union_value()),
                    },
                )
            }
            Message::ChannelAnnouncement {
                short_channel_id,
                node_a_id,
                node_b_id,
            } => {
                let node_a_id = flat::PubKey(node_a_id.serialize());
                let node_b_id = flat::PubKey(node_b_id.serialize());
                let channel_announcement = flat::ChannelAnnouncement::create(
                    builder,
                    &flat::ChannelAnnouncementArgs {
                        short_channel_id,
                        node_a_id: Some(&node_a_id),
                        node_b_id: Some(&node_b_id),
                    },
                );
                flat::GossipRecord::create(
                    builder,
                    &flat::GossipRecordArgs {
                        received_at: msg.received_at.into(),
                        msg_type: flat::Message::ChannelAnnouncement,
                        msg: Some(channel_announcement.as_union_value()),
                    },
                )
            }
            Message::ChannelUpdate {
                short_channel_id,
                update_counter,
                channel_enabled,
                direction,
                cltv_expiry_delta,
                htlc_minimum_msat,
                htlc_maximum_msat,
                fee_base_msat,
                fee_proportional_millionths,
            } => {
                let channel_announcement = flat::ChannelUpdate::create(
                    builder,
                    &flat::ChannelUpdateArgs {
                        short_channel_id,
                        update_counter,
                        channel_enabled,
                        cltv_expiry_delta,
                        direction: match direction {
                            ChannelDirection::AToB => flat::ChannelDirection::AToB,
                            ChannelDirection::BToA => flat::ChannelDirection::BToA,
                        },
                        htlc_minimum_msat: htlc_minimum_msat.into(),
                        htlc_maximum_msat: htlc_maximum_msat.map(u64::from).unwrap_or(0),
                        fee_base_msat: fee_base_msat.into(),
                        fee_proportional_millionths,
                    },
                );
                flat::GossipRecord::create(
                    builder,
                    &flat::GossipRecordArgs {
                        received_at: msg.received_at.into(),
                        msg_type: flat::Message::ChannelAnnouncement,
                        msg: Some(channel_announcement.as_union_value()),
                    },
                )
            }
        };
        builder.finish(msg, None);
        FinishedBytes(builder.finished_data())
    }
}

#[inline]
pub fn hex_str(value: &[u8]) -> String {
    let mut res = String::with_capacity(64);
    for v in value {
        res += &format!("{:02x}", v);
    }
    res
}

impl<'a> From<flat::GossipRecord<'a>> for Option<proto::LnGossip> {
    fn from(record: flat::GossipRecord) -> Self {
        let msg = match record.msg_type() {
            flat::Message::NodeAnnouncement => {
                let node_announcement = record.msg_as_node_announcement().unwrap();
                proto::ln_gossip::Message::NodeAnnouncement(proto::NodeAnnouncement {
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
                    update_counter: channel_update.update_counter(),
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
