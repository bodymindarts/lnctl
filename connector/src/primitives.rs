use bitcoin::secp256k1::{PublicKey, SecretKey};
use std::{
    fmt::{self, Display},
    ops::Deref,
    str::FromStr,
};
use uuid::Uuid;

macro_rules! wrapper {
    ($name:ident, $wrapped:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
        #[repr(transparent)]
        pub struct $name($wrapped);
        impl From<$wrapped> for $name {
            fn from(v: $wrapped) -> Self {
                $name(v)
            }
        }
        impl From<$name> for $wrapped {
            fn from($name(v): $name) -> Self {
                v
            }
        }
        impl Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                self.0.fmt(f)
            }
        }
        impl Deref for $name {
            type Target = $wrapped;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

wrapper! { ConnectorId, Uuid }
wrapper! { ConnectorSecret, SecretKey }
wrapper! { ConnectorPubKey, PublicKey }
wrapper! { MonitoredNodeId, PublicKey }
impl FromStr for MonitoredNodeId {
    type Err = <PublicKey as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<PublicKey>()?))
    }
}

wrapper! { NodeId, PublicKey }
wrapper! { MilliSatoshi, u64 }
impl From<u32> for MilliSatoshi {
    fn from(v: u32) -> Self {
        Self(v as u64)
    }
}
