pub mod flat {
    pub use crate::shared_generated::*;
    pub mod channels_archive {
        use super::*;
        include!("../../../flatbuffers/gen/coordinator/channels_archive_generated.rs");
    }
}

mod convert;
mod error;
mod keys;

use std::path::PathBuf;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use zerocopy::*;

use crate::{bus::*, config};
use ::shared::proto;
use convert::FinishedBytes;
pub use error::*;
use keys::*;

pub(crate) struct Db {
    _inner: sled::Db,
    channels: sled::Tree,
}

impl Db {
    pub fn new(data_dir: &PathBuf, bus: CoordinatorBus) -> anyhow::Result<Self> {
        let db: sled::Db = sled::open(format!("{}/sled", data_dir.display()))?;
        let channels = db.open_tree("monitored-channels-archive")?;
        Self::persist_monitored_channels(channels.clone(), bus.clone());
        Ok(Self {
            _inner: db,
            channels,
        })
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

    pub fn list_monitored_channel_snapshots<T>(&self, short_channel_id: u64) -> DbResult<Vec<T>>
    where
        for<'a> T: TryFrom<flat::channels_archive::MonitoredChannelState<'a>>,
    {
        let (start, end) = ChannelArchiveKey::range(short_channel_id);
        let mut res = Vec::new();
        for item in self.channels.range(start.as_bytes()..end.as_bytes()) {
            let (_, value) = item?;
            let bytes = value.as_ref();
            match flat::channels_archive::root_as_monitored_channel_state(bytes) {
                Ok(record) => {
                    if let Ok(record) = T::try_from(record) {
                        res.push(record);
                    }
                }
                Err(e) => eprintln!("Couldn't de-serialize archived channel state: {}", e),
            }
        }

        Ok(res)
    }
}
