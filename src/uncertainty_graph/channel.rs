use lightning::routing::network_graph::{NodeId, RoutingFees};

pub type ChannelId = u64;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Satoshis(u64);
impl From<u64> for Satoshis {
    fn from(inner: u64) -> Self {
        Self(inner)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct MilliSatoshis(u64);
impl From<u64> for MilliSatoshis {
    fn from(inner: u64) -> Self {
        Self(inner)
    }
}
impl From<&u64> for MilliSatoshis {
    fn from(inner: &u64) -> Self {
        Self(*inner)
    }
}
impl From<&Satoshis> for u64 {
    fn from(Satoshis(ret): &Satoshis) -> Self {
        *ret
    }
}

#[derive(Debug, Clone)]
pub struct ChannelDirectionInfo {
    pub enabled: bool,
    pub fees: RoutingFees,
    pub send_min: MilliSatoshis,
    pub send_max: Option<MilliSatoshis>,
}

pub struct UncertaintyChannel {
    pub node_a: NodeId,
    pub node_b: NodeId,
    pub total_capacity: Option<Satoshis>,
    pub a_to_b_info: Option<ChannelDirectionInfo>,
    pub b_to_a_info: Option<ChannelDirectionInfo>,
}
