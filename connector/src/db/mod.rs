pub mod flat {
    pub use crate::shared_generated::*;
    pub mod gossip {
        use super::*;
        include!("../../../flatbuffers/gen/connector/gossip_generated.rs");
    }
    pub mod channels {
        use super::*;
        include!("../../../flatbuffers/gen/connector/channels_generated.rs");
    }
}

mod convert;
mod keys;

use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use zerocopy::*;

use crate::{bus::*, config};
use convert::FinishedBytes;
use keys::*;

pub(crate) struct Db {
    _inner: sled::Db,
    gossip: sled::Tree,
}

impl Db {
    pub fn new(data_dir: &PathBuf, bus: ConnectorBus) -> anyhow::Result<Self> {
        let db: sled::Db = sled::open(format!("{}/sled", data_dir.display()))?;
        let gossip = db.open_tree("gossip")?;
        let gossip_db = gossip.clone();
        let gossip_bus = bus.clone();
        tokio::spawn(async move {
            let mut stream = gossip_bus.subscribe::<LdkGossip>().await;
            let mut buffer = flatbuffers::FlatBufferBuilder::new();
            while let Some(msg) = stream.next().await {
                let key = GossipMessageKey::from(&msg);
                let finished_bytes = FinishedBytes::from((&mut buffer, msg));
                if let Err(e) = gossip_db.insert(key.as_bytes(), *finished_bytes) {
                    eprintln!("Couldn't persist gossip: {}", e);
                }
            }
        });
        let channel_db = db.open_tree("channels")?;
        tokio::spawn(async move {
            let mut stream = bus.subscribe::<ChannelScrape>().await;
            let mut buffer = flatbuffers::FlatBufferBuilder::new();
            let mut updated = false;
            while let Some(msg) = stream.next().await {
                updated = false;
                let key = ChannelStateKey::from(&msg);
                let finished_bytes = FinishedBytes::from((&mut buffer, msg));
                if let Err(e) = channel_db.update_and_fetch(key.as_bytes(), |existing| {
                    if let Some(existing) = existing {
                        if existing != *finished_bytes {
                            updated = true;
                        }
                    } else {
                        updated = true;
                    }
                    Some(*finished_bytes)
                }) {
                    eprintln!("Couldn't persist channel: {}", e);
                }
            }
        });
        Ok(Self { _inner: db, gossip })
    }

    pub fn load_gossip<T>(&self) -> impl Stream<Item = T>
    where
        for<'a> T: TryFrom<flat::gossip::GossipRecord<'a>> + Send + Sync + 'static,
    {
        let mut iter = self.gossip.iter();
        let (sender, receiver) = mpsc::channel(config::DEFAULT_CHANNEL_SIZE);
        tokio::spawn(async move {
            let mut to_send = None;
            while let Some(Ok((_, value))) = iter.next() {
                let bytes = value.as_ref();
                if let Ok(record) = flat::gossip::root_as_gossip_record(bytes) {
                    to_send = T::try_from(record).ok();
                    if let Some(msg) = to_send.take() {
                        if let Err(e) = sender.send(msg).await {
                            eprintln!("Couldn't send loaded gossip: {}", e);
                        }
                    }
                }
            }
        });
        ReceiverStream::new(receiver)
    }
}
