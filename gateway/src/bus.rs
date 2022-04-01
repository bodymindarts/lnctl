use ::shared::proto;
use shared::{bus::MessageBus, primitives::*};

#[derive(Clone, Debug)]
pub(crate) struct ConnectorMessage {
    pub connector_id: ConnectorId,
    pub monitored_node_id: MonitoredNodeId,
    pub received_at: UnixTimestampSecs,
    pub node_event: proto::NodeEvent,
}

#[derive(Clone, Debug)]
pub(crate) enum BusMessage {
    ConnectorMessage(ConnectorMessage),
}

pub(crate) type GatewayBus = MessageBus<BusMessage>;

pub(crate) struct ConnectorMsgSub<T> {
    pub msg: ConnectorMessage,
    _phantom: std::marker::PhantomData<T>,
}

mod convert {
    use super::*;
    use shared::bus::*;

    impl From<ConnectorMessage> for BusMessage {
        fn from(msg: ConnectorMessage) -> Self {
            BusMessage::ConnectorMessage(msg)
        }
    }

    impl BusSubscriber<BusMessage> for ConnectorMsgSub<proto::MonitoredChannelUpdate> {
        fn filter(msg: &BusMessage) -> bool {
            if let BusMessage::ConnectorMessage(msg) = msg {
                if let Some(proto::node_event::Event::ChannelUpdate(_)) = msg.node_event.event {
                    return true;
                }
            }
            false
        }

        fn convert(msg: BusMessage) -> Option<Self> {
            if let BusMessage::ConnectorMessage(msg) = msg {
                Some(ConnectorMsgSub {
                    msg,
                    _phantom: std::marker::PhantomData,
                })
            } else {
                None
            }
        }
    }
}
