use serde::Serialize;

#[derive(Serialize)]
pub struct ConnectorInfo {
    pub id: String,
    pub monitored_node_id: String,
    pub r#type: String,
}

#[derive(Serialize)]
pub struct ChannelSettings {
    chan_reserve_sat: u64,
    htlc_minimum_msat: u64,
}

#[derive(Serialize)]
pub struct ChannelSnapshot {
    pub timestamp: u64,
    pub short_channel_id: u64,
    pub local_node_id: String,
    pub remote_node_id: String,
    pub active: bool,
    pub private: bool,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub unsettled_balance: u64,
    pub local_channel_settings: Option<ChannelSettings>,
    pub remote_channel_settings: Option<ChannelSettings>,
}

#[derive(Serialize)]
pub struct ChannelHistory {
    snapshots: Vec<ChannelSnapshot>,
}

mod convert {
    use ::shared::proto;
    impl From<proto::ConnectorInfo> for super::ConnectorInfo {
        fn from(proto: proto::ConnectorInfo) -> Self {
            super::ConnectorInfo {
                id: proto.id,
                monitored_node_id: proto.monitored_node_id,
                r#type: proto.r#type,
            }
        }
    }

    impl From<proto::ListMonitoredChannelSnapshotsResponse> for super::ChannelHistory {
        fn from(proto: proto::ListMonitoredChannelSnapshotsResponse) -> Self {
            let mut snapshots = Vec::new();
            for snapshot in proto.snapshots {
                if let Some(state) = snapshot.channel_state {
                    snapshots.push(super::ChannelSnapshot {
                        timestamp: snapshot.timestamp,
                        short_channel_id: state.short_channel_id,
                        local_node_id: state.local_node_id,
                        remote_node_id: state.remote_node_id,
                        active: state.active,
                        private: state.private,
                        capacity: state.capacity,
                        local_balance: state.local_balance,
                        remote_balance: state.remote_balance,
                        unsettled_balance: state.unsettled_balance,
                        local_channel_settings: state.local_channel_settings.map(|settings| {
                            super::ChannelSettings {
                                chan_reserve_sat: settings.chan_reserve_sat,
                                htlc_minimum_msat: settings.htlc_minimum_msat,
                            }
                        }),
                        remote_channel_settings: state.remote_channel_settings.map(|settings| {
                            super::ChannelSettings {
                                chan_reserve_sat: settings.chan_reserve_sat,
                                htlc_minimum_msat: settings.htlc_minimum_msat,
                            }
                        }),
                    });
                }
            }
            super::ChannelHistory { snapshots }
        }
    }
}
