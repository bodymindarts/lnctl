use lightning::ln::msgs::*;

use crate::server::proto;
use shared::{bus::MessageBus, primitives::UnixTimestampSecs};

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
pub(crate) enum BusMessage {
    LdkGossip(LdkGossip),
    NodeEvent(proto::NodeEvent),
}

pub(crate) type ConnectorBus = MessageBus<BusMessage>;

mod convert {
    use super::*;
    use shared::bus::*;

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
}
