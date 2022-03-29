use super::flat;
use crate::bus::*;

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
                let node_announcement = flat::gossip::NodeAnnouncement::create(
                    builder,
                    &flat::gossip::NodeAnnouncementArgs {
                        timestamp,
                        node_id: Some(&pubkey),
                    },
                );
                flat::gossip::GossipRecord::create(
                    builder,
                    &flat::gossip::GossipRecordArgs {
                        received_at: received_at.into(),
                        msg_type: flat::gossip::Message::NodeAnnouncement,
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
                let channel_announcement = flat::gossip::ChannelAnnouncement::create(
                    builder,
                    &flat::gossip::ChannelAnnouncementArgs {
                        short_channel_id,
                        node_a_id: Some(&node_a_id),
                        node_b_id: Some(&node_b_id),
                    },
                );
                flat::gossip::GossipRecord::create(
                    builder,
                    &flat::gossip::GossipRecordArgs {
                        received_at: received_at.into(),
                        msg_type: flat::gossip::Message::ChannelAnnouncement,
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
                    flat::gossip::ChannelDirection::BToA
                } else {
                    flat::gossip::ChannelDirection::AToB
                };
                let channel_update = flat::gossip::ChannelUpdate::create(
                    builder,
                    &flat::gossip::ChannelUpdateArgs {
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
                flat::gossip::GossipRecord::create(
                    builder,
                    &flat::gossip::GossipRecordArgs {
                        received_at: received_at.into(),
                        msg_type: flat::gossip::Message::ChannelUpdate,
                        msg: Some(channel_update.as_union_value()),
                    },
                )
            }
        };
        builder.finish(msg, None);
        FinishedBytes(builder.finished_data())
    }
}
impl<'a> From<(&'a mut flatbuffers::FlatBufferBuilder<'_>, ChannelScrape)> for FinishedBytes<'a> {
    fn from((builder, msg): (&'a mut flatbuffers::FlatBufferBuilder<'_>, ChannelScrape)) -> Self {
        builder.reset();
        let ChannelScrape {
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
                            chan_reserve_sat: local_channel_reserve_sat,
                            min_htlc_msat: local_htlc_minimum_msat,
                        },
                    remote_channel_settings:
                        ChannelSettings {
                            chan_reserve_sat: remote_channel_reserve_sat,
                            min_htlc_msat: remote_htlc_minimum_msat,
                        },
                },
        } = msg;
        let local_channel_settings = flat::channels::ChannelSettings::create(
            builder,
            &flat::channels::ChannelSettingsArgs {
                chan_reserve_sat: local_channel_reserve_sat.into(),
                min_htlc_msat: local_htlc_minimum_msat.into(),
            },
        );
        let remote_channel_settings = flat::channels::ChannelSettings::create(
            builder,
            &flat::channels::ChannelSettingsArgs {
                chan_reserve_sat: remote_channel_reserve_sat.into(),
                min_htlc_msat: remote_htlc_minimum_msat.into(),
            },
        );
        let local_node_id = flat::PubKey(local_node_id.serialize());
        let remote_node_id = flat::PubKey(remote_node_id.serialize());
        let channel_state = flat::channels::ChannelState::create(
            builder,
            &flat::channels::ChannelStateArgs {
                short_channel_id,
                local_node_id: Some(&local_node_id),
                remote_node_id: Some(&remote_node_id),
                active,
                private,
                capacity: capacity.into(),
                local_balance: local_balance.into(),
                remote_balance: remote_balance.into(),
                unsettled_balance: unsettled_balance.into(),
                local_channel_settings: Some(local_channel_settings),
                remote_channel_settings: Some(remote_channel_settings),
            },
        );
        let msg = flat::channels::ChannelScrape::create(
            builder,
            &flat::channels::ChannelScrapeArgs {
                scrape_timestamp: scraped_at.into(),
                state: Some(channel_state),
            },
        );
        builder.finish(msg, None);
        FinishedBytes(builder.finished_data())
    }
}
