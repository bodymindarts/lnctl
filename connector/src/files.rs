use anyhow::Context;
use bitcoin::secp256k1::PublicKey;
use std::{
    fs,
    path::{Path, PathBuf},
    process,
};
use uuid::Uuid;

const DEFAULT_DATA_DIR: &str = ".lnctl/connector";
const UUID_FILE_NAME: &str = "connector-id";
const NODE_PUBKEY_FILE_NAME: &str = "node-pubkey";

pub fn init(path: Option<PathBuf>, pubkey: &PublicKey) -> anyhow::Result<Uuid> {
    let path = path.unwrap_or_else(|| PathBuf::from(DEFAULT_DATA_DIR));
    fs::create_dir_all(&path).context("failed to create data dir")?;
    fs::write(format!("{}/pid", path.display()), process::id().to_string())?;

    let uuid_file_name = format!("{}/{}", path.display(), UUID_FILE_NAME);
    let uuid = if let Ok(content) = fs::read_to_string(&uuid_file_name) {
        if let Ok(uuid) = Uuid::parse_str(&content).context("Couldn't parse uuid") {
            uuid
        } else {
            let uuid = Uuid::new_v4();
            fs::write(uuid_file_name, uuid.to_string()).context("couldn't write uuid file")?;
            uuid
        }
    } else {
        let uuid = Uuid::new_v4();
        fs::write(uuid_file_name, uuid.to_string()).context("couldn't write uuid file")?;
        uuid
    };

    let node_pubkey_file_name = format!("{}/{}", path.display(), NODE_PUBKEY_FILE_NAME);
    if Path::new(&node_pubkey_file_name).exists() {
        if pubkey
            != &fs::read_to_string(&node_pubkey_file_name)
                .context("failed to read node pubkey")?
                .parse::<PublicKey>()
                .context("failed to parse node pubkey")?
        {
            anyhow::bail!("node pubkey does not match")
        }
    } else {
        fs::write(node_pubkey_file_name, pubkey.to_string())?;
    }

    Ok(uuid)
}
