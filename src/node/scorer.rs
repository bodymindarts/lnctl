use super::persistence;
use lightning::routing::scoring::Scorer;
use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

pub(crate) fn init_scorer(data_dir: &Path) -> Arc<Mutex<Scorer>> {
    let scorer_path = format!("{}/scorer", data_dir.display());
    let scorer = Arc::new(Mutex::new(persistence::read_scorer(Path::new(
        &scorer_path,
    ))));
    let scorer_persist = Arc::clone(&scorer);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(600));
        loop {
            interval.tick().await;
            if persistence::persist_scorer(Path::new(&scorer_path), &scorer_persist.lock().unwrap())
                .is_err()
            {
                // Persistence errors here are non-fatal as channels will be re-scored as payments
                // fail, but they may indicate a disk error which could be fatal elsewhere.
                eprintln!("Warning: Failed to persist scorer, check your disk and permissions");
            }
        }
    });
    scorer
}
