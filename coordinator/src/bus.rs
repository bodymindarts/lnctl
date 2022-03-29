use crate::connector::proto;
use shared::{bus::MessageBus, primitives::*};

#[derive(Clone, Debug)]
pub(crate) enum BusMessage {
    ConnectorMessage {
        connector_id: ConnectorId,
        monitored_node_id: MonitoredNodeId,
        node_event: proto::NodeEvent,
    },
}

pub(crate) type CoordinatorBus = MessageBus<BusMessage>;
