pub mod flat {
    include!("../../../flatbuffers/gen/connector/gossip_generated.rs");
}

mod convert;
mod keys;

use std::path::PathBuf;
use tokio::sync::mpsc;
use zerocopy::*;

use crate::gossip::GossipMessage;
use crate::server::proto;
use convert::FinishedBytes;
use keys::GossipMessageKey;

pub struct Db {
    _inner: sled::Db,
    gossip: sled::Tree,
}

impl Db {
    pub fn new(data_dir: &PathBuf) -> anyhow::Result<Self> {
        let db: sled::Db = sled::open(format!("{}/sled", data_dir.display()))?;
        let gossip = db.open_tree("gossip")?;
        Ok(Self { _inner: db, gossip })
    }

    pub fn forward_gossip(
        &self,
        mut receiver: mpsc::Receiver<GossipMessage>,
    ) -> mpsc::Receiver<GossipMessage> {
        let (sender, new_receiver) = mpsc::channel(50);
        let gossip_db = self.gossip.clone();
        let mut buffer = flatbuffers::FlatBufferBuilder::new();
        tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                let finished_bytes = FinishedBytes::from((&mut buffer, &msg));
                let key = GossipMessageKey::from(&msg);
                if let Err(e) = sender.send(msg).await {
                    eprintln!("Couldn't forward gossip: {}", e);
                }
                if let Err(e) = gossip_db.compare_and_swap(
                    key.as_bytes(),
                    None as Option<&[u8]>,
                    Some(*finished_bytes),
                ) {
                    eprintln!("Couldn't persist gossip: {}", e);
                }
            }
        });
        new_receiver
    }

    pub fn iter_gossip(&self, sender: mpsc::Sender<proto::LnGossip>) {
        let mut iter = self.gossip.iter();
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
    }
}
