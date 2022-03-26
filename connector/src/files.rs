use anyhow::Context;
use bitcoin::{
    network::constants::Network,
    secp256k1::{PublicKey, Secp256k1, SecretKey, Signing},
    util::bip32::{ChildNumber, ExtendedPrivKey},
};
use rand::{thread_rng, Rng};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process,
};

use shared::primitives::*;

const CONNECTOR_SEED_FILE: &str = "seed";
const UUID_FILE_NAME: &str = "connector-id";
const NODE_PUBKEY_FILE_NAME: &str = "node-pubkey";

pub fn init(
    path: PathBuf,
    node_id: &MonitoredNodeId,
) -> anyhow::Result<(ConnectorId, ConnectorPubKey, ConnectorSecret)> {
    fs::create_dir_all(&path).context("failed to create data dir")?;
    fs::write(format!("{}/pid", path.display()), process::id().to_string())?;

    let secp_ctx = Secp256k1::new();
    let secret_key = init_node_secret(&path, &secp_ctx)?;
    let connector_pubkey = PublicKey::from_secret_key(&secp_ctx, &secret_key);
    let uuid = uuid::Builder::from_slice(&connector_pubkey.serialize()[0..16])?
        .set_variant(uuid::Variant::RFC4122)
        .set_version(uuid::Version::Random)
        .build();

    let uuid_file_name = format!("{}/{}", path.display(), UUID_FILE_NAME);
    fs::write(uuid_file_name, uuid.to_string()).context("couldn't write uuid file")?;

    let node_pubkey_file_name = format!("{}/{}", path.display(), NODE_PUBKEY_FILE_NAME);
    if Path::new(&node_pubkey_file_name).exists() {
        if node_id
            != &fs::read_to_string(&node_pubkey_file_name)
                .context("failed to read node pubkey")?
                .parse::<MonitoredNodeId>()
                .context("failed to parse node pubkey")?
        {
            anyhow::bail!("node pubkey does not match")
        }
    } else {
        fs::write(node_pubkey_file_name, node_id.to_string())?;
    }
    Ok((uuid.into(), connector_pubkey.into(), secret_key.into()))
}

fn init_node_secret<C: Signing>(
    path: &PathBuf,
    secp_ctx: &Secp256k1<C>,
) -> anyhow::Result<SecretKey> {
    let keys_seed_path = format!("{}/{}", path.display(), CONNECTOR_SEED_FILE);
    let keys_seed = if let Ok(seed) = fs::read(keys_seed_path.clone()) {
        assert_eq!(seed.len(), 32);
        let mut key = [0; 32];
        key.copy_from_slice(&seed);
        key
    } else {
        let mut key = [0; 32];
        thread_rng().fill_bytes(&mut key);
        match fs::File::create(keys_seed_path.clone()) {
            Ok(mut f) => {
                f.write_all(&key)
                    .expect("Failed to write node keys seed to disk");
                f.sync_all().expect("Failed to sync node keys seed to disk");
            }
            Err(e) => {
                println!(
                    "ERROR: Unable to create keys seed file {}: {}",
                    keys_seed_path, e
                );
                return Err(anyhow::anyhow!("Failed to create keys seed file"));
            }
        }
        key
    };
    // Note that when we aren't serializing the key, network doesn't matter
    match ExtendedPrivKey::new_master(Network::Testnet, &keys_seed) {
        Ok(master_key) => Ok(master_key
            .ckd_priv(&secp_ctx, ChildNumber::from_hardened_idx(0).unwrap())
            .expect("Your RNG is busted")
            .private_key
            .key),
        Err(_) => anyhow::bail!("Your rng is busted"),
    }
}
