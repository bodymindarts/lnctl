pub mod shared {
    tonic::include_proto!("shared");
}

pub mod connector {
    tonic::include_proto!("connector");
}

pub mod gateway {
    tonic::include_proto!("gateway");
}

pub use connector::*;
pub use gateway::*;
pub use shared::*;

mod convert {
    use super::*;

    impl From<ConnectorType> for String {
        fn from(t: ConnectorType) -> Self {
            match t {
                ConnectorType::Lnd => "lnd".to_string(),
            }
        }
    }
}
