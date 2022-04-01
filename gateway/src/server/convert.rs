use crate::db::{flat, DbError};
use ::shared::proto;

impl<'a> TryFrom<flat::channels_archive::MonitoredChannelState<'a>>
    for proto::MonitoredChannelSnapshot
{
    type Error = ();

    fn try_from(
        record: flat::channels_archive::MonitoredChannelState,
    ) -> Result<Self, Self::Error> {
        let state = record.state().ok_or(())?;
        let local_channel_settings = state.local_channel_settings().ok_or(())?;
        let remote_channel_settings = state.remote_channel_settings().ok_or(())?;
        let msg = proto::MonitoredChannelSnapshot {
            timestamp: record.scrape_timestamp(),
            channel_state: Some(proto::ChannelState {
                short_channel_id: state.short_channel_id(),
                local_node_id: state.local_node_id().ok_or(())?.into(),
                remote_node_id: state.remote_node_id().ok_or(())?.into(),
                active: state.active(),
                private: state.private(),
                capacity: state.capacity(),
                local_balance: state.local_balance(),
                remote_balance: state.remote_balance(),
                unsettled_balance: state.unsettled_balance(),
                local_channel_settings: Some(proto::ChannelSettings {
                    chan_reserve_sat: local_channel_settings.chan_reserve_sat(),
                    htlc_minimum_msat: local_channel_settings.htlc_minimum_msat(),
                }),
                remote_channel_settings: Some(proto::ChannelSettings {
                    chan_reserve_sat: remote_channel_settings.chan_reserve_sat(),
                    htlc_minimum_msat: remote_channel_settings.htlc_minimum_msat(),
                }),
            }),
        };
        Ok(msg)
    }
}

impl From<DbError> for tonic::Status {
    fn from(_: DbError) -> Self {
        tonic::Status::new(tonic::Code::Internal, "Database error")
    }
}
