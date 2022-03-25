use super::flat;
use crate::gossip::{ChannelDirection, GossipMessage, Message};

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
        match msg.msg {
            Message::NodeAnnouncement { node_id } => {
                let pubkey = flat::PubKey(node_id.serialize());
                let node_announcement = flat::NodeAnnouncement::create(
                    builder,
                    &flat::NodeAnnouncementArgs {
                        node_id: Some(&pubkey),
                    },
                );
                let msg = flat::GossipRecord::create(
                    builder,
                    &flat::GossipRecordArgs {
                        received_at: msg.received_at.into(),
                        msg_type: flat::Message::NodeAnnouncement,
                        msg: Some(node_announcement.as_union_value()),
                    },
                );
                builder.finish(msg, None);
                FinishedBytes(builder.finished_data())
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
                let msg = flat::GossipRecord::create(
                    builder,
                    &flat::GossipRecordArgs {
                        received_at: msg.received_at.into(),
                        msg_type: flat::Message::ChannelAnnouncement,
                        msg: Some(channel_announcement.as_union_value()),
                    },
                );
                builder.finish(msg, None);
                FinishedBytes(builder.finished_data())
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
                let msg = flat::GossipRecord::create(
                    builder,
                    &flat::GossipRecordArgs {
                        received_at: msg.received_at.into(),
                        msg_type: flat::Message::ChannelAnnouncement,
                        msg: Some(channel_announcement.as_union_value()),
                    },
                );
                builder.finish(msg, None);
                FinishedBytes(builder.finished_data())
            }
        }
    }
}
