use anyhow::anyhow;
use lightning::{chain::keysinterface::KeysManager, util::ser::Writer};
use rand::{thread_rng, Rng};
use std::path::Path;
use std::sync::Arc;
use std::{
    fs::{self, File},
    time::SystemTime,
};

pub fn init_keys_manager(data_dir: &Path) -> Result<Arc<KeysManager>, anyhow::Error> {
    let keys_seed_path = format!("{}/keys_seed", data_dir.display());
    let keys_seed = if let Ok(seed) = fs::read(keys_seed_path.clone()) {
        assert_eq!(seed.len(), 32);
        let mut key = [0; 32];
        key.copy_from_slice(&seed);
        key
    } else {
        let mut key = [0; 32];
        thread_rng().fill_bytes(&mut key);
        match File::create(keys_seed_path.clone()) {
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
                return Err(anyhow!("Failed to create keys seed file"));
            }
        }
        key
    };
    let cur = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    Ok(Arc::new(KeysManager::new(
        &keys_seed,
        cur.as_secs(),
        cur.subsec_nanos(),
    )))
}
