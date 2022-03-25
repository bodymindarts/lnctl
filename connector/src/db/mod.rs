pub mod flat {
    include!("../../../flatbuffers/gen/connector/gossip_generated.rs");
}

mod convert;

use std::path::PathBuf;
use tokio::sync::mpsc;
use zerocopy::*;

use crate::gossip::GossipMessage;
use convert::FinishedBytes;

pub struct Db {
    inner: sled::Db,
    gossip: sled::Tree,
}

impl Db {
    pub fn new(data_dir: &PathBuf) -> anyhow::Result<Self> {
        let db: sled::Db = sled::open(format!("{}/sled", data_dir.display()))?;
        let gossip = db.open_tree("gossip")?;
        Ok(Self { inner: db, gossip })
    }

    pub fn forward_gossip(
        &self,
        mut receiver: mpsc::Receiver<GossipMessage>,
    ) -> mpsc::Receiver<GossipMessage> {
        let (sender, new_receiver) = mpsc::channel(50);
        let gossip_db = self.gossip.clone();
        let db = self.inner.clone();
        let mut buffer = flatbuffers::FlatBufferBuilder::new();
        tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                let finished_bytes: FinishedBytes = (&mut buffer, &msg).into();
                if let Err(e) = sender.send(msg).await {
                    eprintln!("Couldn't forward gossip: {}", e);
                }
                let key = InsertKey {
                    id: U64::new(db.generate_id().expect("generate id")),
                };
                if let Err(e) = gossip_db.insert(key.as_bytes(), *finished_bytes) {
                    eprintln!("Couldn't persist gossip: {}", e);
                }
            }
        });
        new_receiver
    }
}

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
pub struct InsertKey {
    pub id: U64<BigEndian>,
}
