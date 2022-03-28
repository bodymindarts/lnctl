pub mod flat {
    include!("../../../flatbuffers/gen/connector/gossip_generated.rs");
}

mod convert;
mod keys;

use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use zerocopy::*;

use crate::server::proto;
use crate::{bus::*, config};
use convert::FinishedBytes;
use keys::GossipMessageKey;

pub(crate) struct Db {
    _inner: sled::Db,
    gossip: sled::Tree,
}

impl Db {
    pub fn new(data_dir: &PathBuf, bus: ConnectorBus) -> anyhow::Result<Self> {
        let db: sled::Db = sled::open(format!("{}/sled", data_dir.display()))?;
        let gossip = db.open_tree("gossip")?;
        let gossip_db = gossip.clone();
        tokio::spawn(async move {
            let mut stream = bus.subscribe::<LdkGossip>().await;
            let mut buffer = flatbuffers::FlatBufferBuilder::new();
            while let Some(msg) = stream.next().await {
                let key = GossipMessageKey::from(&msg);
                let finished_bytes = FinishedBytes::from((&mut buffer, msg));
                if let Err(e) = gossip_db.compare_and_swap(
                    key.as_bytes(),
                    None as Option<&[u8]>,
                    Some(*finished_bytes),
                ) {
                    eprintln!("Couldn't persist gossip: {}", e);
                }
            }
        });
        Ok(Self { _inner: db, gossip })
    }

    pub fn load_gossip(&self) -> impl Stream<Item = proto::LnGossip> {
        let mut iter = self.gossip.iter();
        let (sender, receiver) = mpsc::channel(config::DEFAULT_CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(Ok((_, value))) = iter.next() {
                let bytes = value.as_ref();
                if let Ok(record) = flat::root_as_gossip_record(bytes) {
                    if let Some(msg) = Option::<proto::LnGossip>::from(record) {
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
