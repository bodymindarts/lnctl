pub mod shared {
    tonic::include_proto!("shared");
}

pub mod connector {
    tonic::include_proto!("connector");
}

pub mod coordinator {
    tonic::include_proto!("coordinator");
}

pub use connector::*;
pub use coordinator::*;
pub use shared::*;

mod convert {
    use super::*;
    use crate::utils;

    impl From<ConnectorType> for String {
        fn from(t: ConnectorType) -> Self {
            match t {
                ConnectorType::Lnd => "lnd".to_string(),
            }
        }
    }
}
