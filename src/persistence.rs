use anyhow::{anyhow, Context};
use bitcoin::hash_types::BlockHash;
use lightning::{
    routing::network_graph::NetworkGraph,
    util::ser::{Readable, Writeable},
};
use lightning_persister::FilesystemPersister;
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn init_persister(data_dir: &PathBuf) -> Result<Arc<FilesystemPersister>, anyhow::Error> {
    fs::create_dir_all(data_dir).context("failed to create data directory")?;
    Ok(Arc::new(FilesystemPersister::new(
        data_dir.as_path().display().to_string(),
    )))
}

pub(crate) fn read_network(path: &Path, genesis_hash: BlockHash) -> NetworkGraph {
    if let Ok(file) = File::open(path) {
        if let Ok(graph) = NetworkGraph::read(&mut BufReader::new(file)) {
            return graph;
        }
    }
    NetworkGraph::new(genesis_hash)
}

pub(crate) fn persist_network(path: &Path, network_graph: &NetworkGraph) -> std::io::Result<()> {
    let mut tmp_path = path.to_path_buf().into_os_string();
    tmp_path.push(".tmp");
    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&tmp_path)?;
    let write_res = network_graph.write(&mut BufWriter::new(file));
    if let Err(e) = write_res.and_then(|_| fs::rename(&tmp_path, path)) {
        let _ = fs::remove_file(&tmp_path);
        Err(e)
    } else {
        Ok(())
    }
}
