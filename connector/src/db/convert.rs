use super::flat;
use crate::{bus::*, server::proto};
use shared::utils::hex_str;

#[repr(transparent)]
pub struct FinishedBytes<'a>(&'a [u8]);
impl<'a> std::ops::Deref for FinishedBytes<'a> {
    type Target = &'a [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<(&'a mut flatbuffers::FlatBufferBuilder<'_>, LdkGossip)> for FinishedBytes<'a> {
    fn from((builder, msg): (&'a mut flatbuffers::FlatBufferBuilder<'_>, LdkGossip)) -> Self {
        use lightning::ln::msgs::*;
        builder.reset();
        let msg = match msg {
            LdkGossip::NodeAnnouncement {
                received_at,
                msg:
                    UnsignedNodeAnnouncement {
                        timestamp, node_id, ..
                    },
            } => {
                let pubkey = flat::PubKey(node_id.serialize());
                let node_announcement = flat::NodeAnnouncement::create(
                    builder,
                    &flat::NodeAnnouncementArgs {
                        timestamp,
                        node_id: Some(&pubkey),
                    },
                );
                flat::GossipRecord::create(
                    builder,
                    &flat::GossipRecordArgs {
                        received_at: received_at.into(),
                        msg_type: flat::Message::NodeAnnouncement,
                        msg: Some(node_announcement.as_union_value()),
                    },
                )
            }
            LdkGossip::ChannelAnnouncement {
                received_at,
                msg:
                    UnsignedChannelAnnouncement {
                        short_channel_id,
                        node_id_1,
                        node_id_2,
                        ..
                    },
            } => {
                let node_a_id = flat::PubKey(node_id_1.serialize());
                let node_b_id = flat::PubKey(node_id_2.serialize());
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
                        received_at: received_at.into(),
                        msg_type: flat::Message::ChannelAnnouncement,
                        msg: Some(channel_announcement.as_union_value()),
                    },
                )
            }
            LdkGossip::ChannelUpdate {
                received_at,
                msg:
                    UnsignedChannelUpdate {
                        flags,
                        short_channel_id,
                        timestamp,
                        cltv_expiry_delta,
                        htlc_minimum_msat,
                        htlc_maximum_msat,
                        fee_base_msat,
                        fee_proportional_millionths,
                        ..
                    },
            } => {
                let channel_enabled = flags & (1 << 1) != (1 << 1);
                let direction = if flags & 1 == 1 {
                    flat::ChannelDirection::BToA
                } else {
                    flat::ChannelDirection::AToB
                };
                let channel_update = flat::ChannelUpdate::create(
                    builder,
                    &flat::ChannelUpdateArgs {
                        short_channel_id,
                        timestamp,
                        channel_enabled,
                        cltv_expiry_delta,
                        direction,
                        htlc_minimum_msat: htlc_minimum_msat.into(),
                        htlc_maximum_msat: match htlc_maximum_msat {
                            OptionalField::Present(v) => v,
                            _ => 0,
                        },
                        fee_base_msat,
                        fee_proportional_millionths,
                    },
                );
                flat::GossipRecord::create(
                    builder,
                    &flat::GossipRecordArgs {
                        received_at: received_at.into(),
                        msg_type: flat::Message::ChannelUpdate,
                        msg: Some(channel_update.as_union_value()),
                    },
                )
            }
        };
        builder.finish(msg, None);
        FinishedBytes(builder.finished_data())
    }
}
