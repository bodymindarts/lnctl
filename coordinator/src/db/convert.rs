use super::flat;
use crate::bus::*;
use shared::{primitives::NodeId, proto};

#[repr(transparent)]
pub struct FinishedBytes<'a>(&'a [u8]);
impl<'a> std::ops::Deref for FinishedBytes<'a> {
    type Target = &'a [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a>
    TryFrom<(
        &'a mut flatbuffers::FlatBufferBuilder<'_>,
        ConnectorMsgSub<proto::MonitoredChannelUpdate>,
    )> for FinishedBytes<'a>
{
    type Error = ();

    fn try_from(
        (builder, ConnectorMsgSub { msg, .. }): (
            &'a mut flatbuffers::FlatBufferBuilder<'_>,
            ConnectorMsgSub<proto::MonitoredChannelUpdate>,
        ),
    ) -> Result<Self, Self::Error> {
        builder.reset();
        if let Some(proto::node_event::Event::ChannelUpdate(update)) = msg.node_event.event {
            let channel_state = update.channel_state.ok_or(())?;
            let monitored_node_id = flat::PubKey(msg.monitored_node_id.serialize());
            let connector_id = flat::ConnectorId(msg.connector_id.as_bytes().clone());
            let remote_node_id = flat::PubKey(
                channel_state
                    .remote_node_id
                    .parse::<NodeId>()
                    .unwrap()
                    .serialize(),
            );
            let local_channel_settings = flat::ChannelSettings::create(
                builder,
                &flat::ChannelSettingsArgs {
                    chan_reserve_sat: channel_state
                        .local_channel_settings
                        .as_ref()
                        .ok_or(())?
                        .chan_reserve_sat,
                    htlc_minimum_msat: channel_state
                        .local_channel_settings
                        .ok_or(())?
                        .htlc_minimum_msat,
                },
            );
            let remote_channel_settings = flat::ChannelSettings::create(
                builder,
                &flat::ChannelSettingsArgs {
                    chan_reserve_sat: channel_state
                        .remote_channel_settings
                        .as_ref()
                        .ok_or(())?
                        .chan_reserve_sat,
                    htlc_minimum_msat: channel_state
                        .remote_channel_settings
                        .ok_or(())?
                        .htlc_minimum_msat,
                },
            );
            let state = flat::ChannelState::create(
                builder,
                &flat::ChannelStateArgs {
                    short_channel_id: channel_state.short_channel_id,
                    local_node_id: Some(&monitored_node_id),
                    remote_node_id: Some(&remote_node_id),
                    active: channel_state.active,
                    private: channel_state.private,
                    capacity: channel_state.capacity,
                    local_balance: channel_state.local_balance,
                    remote_balance: channel_state.remote_balance,
                    unsettled_balance: channel_state.unsettled_balance,
                    local_channel_settings: Some(local_channel_settings),
                    remote_channel_settings: Some(remote_channel_settings),
                },
            );
            let msg = flat::channels_archive::MonitoredChannelState::create(
                builder,
                &flat::channels_archive::MonitoredChannelStateArgs {
                    monitored_node_id: Some(&monitored_node_id),
                    connector_id: Some(&connector_id),
                    archived_timestamp: msg.received_at.into(),
                    scrape_timestamp: update.timestamp,
                    state: Some(state),
                },
            );
            builder.finish(msg, None);
            return Ok(FinishedBytes(builder.finished_data()));
        }
        Err(())
    }
}
