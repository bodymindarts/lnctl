use anyhow::Context;
use bitcoin::{hash_types::BlockHash, secp256k1::PublicKey};
use lightning::{
    routing::{network_graph::NetworkGraph, scoring::Scorer},
    util::ser::{Readable, Writeable},
};
use lightning_persister::FilesystemPersister;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter},
    net::{SocketAddr, ToSocketAddrs},
    path::Path,
    sync::Arc,
};

pub fn init_persister(data_dir: &Path) -> Result<Arc<FilesystemPersister>, anyhow::Error> {
    fs::create_dir_all(data_dir).context("failed to create data directory")?;
    Ok(Arc::new(FilesystemPersister::new(
        data_dir.display().to_string(),
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

pub(crate) fn persist_scorer(path: &Path, scorer: &Scorer) -> std::io::Result<()> {
    let mut tmp_path = path.to_path_buf().into_os_string();
    tmp_path.push(".tmp");
    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&tmp_path)?;
    let write_res = scorer.write(&mut BufWriter::new(file));
    if let Err(e) = write_res.and_then(|_| fs::rename(&tmp_path, path)) {
        let _ = fs::remove_file(&tmp_path);
        Err(e)
    } else {
        Ok(())
    }
}

pub(crate) fn read_scorer(path: &Path) -> Scorer {
    if let Ok(file) = File::open(path) {
        if let Ok(scorer) = Scorer::read(&mut BufReader::new(file)) {
            return scorer;
        }
    }
    Scorer::default()
}

pub(crate) fn read_channel_peer_data(
    data_dir: &Path,
) -> Result<HashMap<PublicKey, SocketAddr>, std::io::Error> {
    let path = format!("{}/channel_peer_data", data_dir.display());
    let mut peer_data = HashMap::new();
    if !Path::new(&path).exists() {
        return Ok(HashMap::new());
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match parse_peer_info(line.unwrap()) {
            Ok((pubkey, socket_addr)) => {
                peer_data.insert(pubkey, socket_addr);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(peer_data)
}

pub(crate) fn parse_peer_info(
    peer_pubkey_and_ip_addr: String,
) -> Result<(PublicKey, SocketAddr), std::io::Error> {
    let mut pubkey_and_addr = peer_pubkey_and_ip_addr.split('@');
    let pubkey = pubkey_and_addr.next();
    let peer_addr_str = pubkey_and_addr.next();
    if peer_addr_str.is_none() || peer_addr_str.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ERROR: incorrectly formatted peer info. Should be formatted as: `pubkey@host:port`",
        ));
    }

    let peer_addr = peer_addr_str
        .unwrap()
        .to_socket_addrs()
        .map(|mut r| r.next());
    if peer_addr.is_err() || peer_addr.as_ref().unwrap().is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ERROR: couldn't parse pubkey@host:port into a socket address",
        ));
    }

    let pubkey = to_compressed_pubkey(pubkey.unwrap());
    if pubkey.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ERROR: unable to parse given pubkey for node",
        ));
    }

    Ok((pubkey.unwrap(), peer_addr.unwrap().unwrap()))
}

fn to_compressed_pubkey(hex: &str) -> Option<PublicKey> {
    let data = match to_vec(&hex[0..33 * 2]) {
        Some(bytes) => bytes,
        None => return None,
    };
    match PublicKey::from_slice(&data) {
        Ok(pk) => Some(pk),
        Err(_) => None,
    }
}

pub fn to_vec(hex: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(hex.len() / 2);

    let mut b = 0;
    for (idx, c) in hex.as_bytes().iter().enumerate() {
        b <<= 4;
        match *c {
            b'A'..=b'F' => b |= c - b'A' + 10,
            b'a'..=b'f' => b |= c - b'a' + 10,
            b'0'..=b'9' => b |= c - b'0',
            _ => return None,
        }
        if (idx & 1) == 1 {
            out.push(b);
            b = 0;
        }
    }

    Some(out)
}
