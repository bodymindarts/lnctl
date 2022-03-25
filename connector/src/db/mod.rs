pub mod flat {
    include!("../../../flatbuffers/gen/gossip_generated.rs");
}

mod convert;

use std::path::PathBuf;

pub struct Db {
    inner: sled::Db,
    gossip: sled::Tree,
}

impl Db {
    pub fn new(data_dir: PathBuf) -> anyhow::Result<Self> {
        let db: sled::Db = sled::open(format!("{}/sled", data_dir.display()))?;
        let gossip = db.open_tree("gossip")?;
        Ok(Self { inner: db, gossip })
    }
}
