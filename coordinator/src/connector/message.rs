use super::client::proto::NodeEvent;
use shared::primitives::*;

pub struct ConnectorMessage {
    pub connector_id: ConnectorId,
    pub monitored_node_id: MonitoredNodeId,
    pub node_event: NodeEvent,
}
