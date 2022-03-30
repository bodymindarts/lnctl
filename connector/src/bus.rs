use lightning::ln::msgs::*;

use crate::server::proto;
use shared::{bus::MessageBus, primitives::*};

#[derive(Clone, Debug)]
pub(crate) enum LdkGossip {
    NodeAnnouncement {
        received_at: UnixTimestampSecs,
        msg: UnsignedNodeAnnouncement,
    },
    ChannelAnnouncement {
        received_at: UnixTimestampSecs,
        msg: UnsignedChannelAnnouncement,
    },
    ChannelUpdate {
        received_at: UnixTimestampSecs,
        msg: UnsignedChannelUpdate,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct ChannelSettings {
    pub chan_reserve_sat: Satoshi,
    pub htlc_minimum_msat: MilliSatoshi,
}

#[derive(Clone, Debug)]
pub(crate) struct ChannelState {
    pub short_channel_id: u64,
    pub local_node_id: MonitoredNodeId,
    pub remote_node_id: NodeId,
    pub active: bool,
    pub private: bool,
    pub capacity: Satoshi,
    pub local_balance: Satoshi,
    pub remote_balance: Satoshi,
    pub unsettled_balance: Satoshi,
    pub local_channel_settings: ChannelSettings,
    pub remote_channel_settings: ChannelSettings,
}

#[derive(Clone, Debug)]
pub(crate) struct ChannelScrape {
    pub scraped_at: UnixTimestampSecs,
    pub state: ChannelState,
}

#[derive(Clone, Debug)]
pub(crate) struct MonitoredChannelUpdate {
    pub scrape: ChannelScrape,
}

#[derive(Clone, Debug)]
pub(crate) enum BusMessage {
    NodeEvent(proto::NodeEvent),
    LdkGossip(LdkGossip),
    ChannelScrape(ChannelScrape),
    ChannelUpdate(MonitoredChannelUpdate),
}

pub(crate) type ConnectorBus = MessageBus<BusMessage>;

mod convert {
    use super::*;
    use shared::bus::*;

    impl From<ChannelState> for BusMessage {
        fn from(channel_state: ChannelState) -> Self {
            BusMessage::ChannelScrape(ChannelScrape {
                scraped_at: UnixTimestampSecs::now(),
                state: channel_state,
            })
        }
    }

    impl From<MonitoredChannelUpdate> for BusMessage {
        fn from(update: MonitoredChannelUpdate) -> Self {
            BusMessage::ChannelUpdate(update)
        }
    }

    impl From<&NodeAnnouncement> for BusMessage {
        fn from(msg: &NodeAnnouncement) -> Self {
            BusMessage::LdkGossip(LdkGossip::NodeAnnouncement {
                received_at: UnixTimestampSecs::now(),
                msg: msg.contents.clone(),
            })
        }
    }
    impl From<&ChannelAnnouncement> for BusMessage {
        fn from(msg: &ChannelAnnouncement) -> Self {
            BusMessage::LdkGossip(LdkGossip::ChannelAnnouncement {
                received_at: UnixTimestampSecs::now(),
                msg: msg.contents.clone(),
            })
        }
    }
    impl From<&ChannelUpdate> for BusMessage {
        fn from(msg: &ChannelUpdate) -> Self {
            BusMessage::LdkGossip(LdkGossip::ChannelUpdate {
                received_at: UnixTimestampSecs::now(),
                msg: msg.contents.clone(),
            })
        }
    }

    impl From<proto::NodeEvent> for BusMessage {
        fn from(proto: proto::NodeEvent) -> Self {
            BusMessage::NodeEvent(proto)
        }
    }

    impl BusSubscriber<BusMessage> for LdkGossip {
        fn filter(msg: &BusMessage) -> bool {
            if let BusMessage::LdkGossip(_) = msg {
                true
            } else {
                false
            }
        }

        fn convert(msg: BusMessage) -> Option<Self> {
            if let BusMessage::LdkGossip(msg) = msg {
                Some(msg)
            } else {
                None
            }
        }
    }

    impl BusSubscriber<BusMessage> for ChannelScrape {
        fn filter(msg: &BusMessage) -> bool {
            if let BusMessage::ChannelScrape(_) = msg {
                true
            } else {
                false
            }
        }

        fn convert(msg: BusMessage) -> Option<Self> {
            if let BusMessage::ChannelScrape(msg) = msg {
                Some(msg)
            } else {
                None
            }
        }
    }
}
