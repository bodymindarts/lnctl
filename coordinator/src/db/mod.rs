pub mod flat {
    pub use crate::shared_generated::*;
    pub mod channels_archive {
        use super::*;
        include!("../../../flatbuffers/gen/coordinator/channels_archive_generated.rs");
    }
}

mod convert;
mod keys;

use std::path::PathBuf;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use zerocopy::*;

use crate::{bus::*, config};
use ::shared::proto;
use convert::FinishedBytes;
use keys::*;

pub(crate) struct Db {
    _inner: sled::Db,
}

impl Db {
    pub fn new(data_dir: &PathBuf, bus: CoordinatorBus) -> anyhow::Result<Self> {
        let db: sled::Db = sled::open(format!("{}/sled", data_dir.display()))?;
        let channels = db.open_tree("monitored-channels-archive")?;
        Self::persist_monitored_channels(channels, bus.clone());
        Ok(Self { _inner: db })
    }

    fn persist_monitored_channels(channels: sled::Tree, bus: CoordinatorBus) {
        tokio::spawn(async move {
            let mut stream = bus
                .subscribe::<ConnectorMsgSub<proto::MonitoredChannelUpdate>>()
                .await;
            let mut buffer = flatbuffers::FlatBufferBuilder::new();
            while let Some(msg) = stream.next().await {
                if let Ok(key) = ChannelArchiveKey::try_from(&msg) {
                    if let Ok(finished_bytes) = FinishedBytes::try_from((&mut buffer, msg)) {
                        if let Err(e) = channels.insert(key.as_bytes(), *finished_bytes) {
                            eprintln!("Couldn't persist gossip: {}", e);
                        }
                    }
                }
            }
        });
    }
}
