use crate::node::hex_utils;
use anyhow::*;
use bitcoin::secp256k1::PublicKey;
use lightning::ln::msgs::NetAddress;
use serde::Deserialize;
use std::{
    env,
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    path::PathBuf,
};

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    #[serde(default = "default_listen_port")]
    pub listen_port: u16,
    pub public_address: Option<SocketAddr>,
    #[serde(skip_deserializing)]
    pub net_address: Option<NetAddress>,
    pub name: Option<String>,
}
impl NodeConfig {
    fn init_net_address(&mut self) {
        if let Some(addr) = self.public_address {
            match addr.ip() {
                IpAddr::V4(ip) => {
                    self.net_address = Some(NetAddress::IPv4 {
                        addr: ip.octets(),
                        port: addr.port(),
                    })
                }
                IpAddr::V6(ip) => {
                    self.net_address = Some(NetAddress::IPv6 {
                        addr: ip.octets(),
                        port: addr.port(),
                    })
                }
            }
        }
    }
    pub fn announced_node_name(&self) -> [u8; 32] {
        match self.name.as_ref() {
            Some(s) => {
                if s.len() > 32 {
                    panic!("Node Alias can not be longer than 32 bytes");
                }
                let mut bytes = [0; 32];
                bytes[..s.len()].copy_from_slice(s.as_bytes());
                bytes
            }
            None => [0; 32],
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "bitcoind")]
    pub bitcoind_config: BitcoindConfig,
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
    pub node: NodeConfig,
    #[serde(default = "default_grpc_port")]
    pub grpc_port: u16,
    pub bootstrap_peers: Option<Vec<String>>,
    #[serde(skip_deserializing)]
    pub peers: Vec<(PublicKey, SocketAddr)>,
}

fn default_data_dir() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push(".lnctl");
    path
}
fn default_listen_port() -> u16 {
    9735
}
fn default_grpc_port() -> u16 {
    5625
}

impl Config {
    pub fn from_path(path: PathBuf) -> Result<Self, anyhow::Error> {
        let config_file = std::fs::read_to_string(path).context("Couldn't read config file")?;
        let mut config: Config =
            serde_yaml::from_str(&config_file).context("Couldn't parse config file")?;
        config.node.init_net_address();
        if let Some(peers) = config.bootstrap_peers.take() {
            for peer in peers {
                config.peers.push(Self::parse_peer_info(peer)?);
            }
        }
        Ok(config)
    }

    fn parse_peer_info(peer_pubkey_and_ip_addr: String) -> anyhow::Result<(PublicKey, SocketAddr)> {
        let mut pubkey_and_addr = peer_pubkey_and_ip_addr.split("@");
        let pubkey = pubkey_and_addr.next();
        let peer_addr_str = pubkey_and_addr.next();
        if peer_addr_str.is_none() || peer_addr_str.is_none() {
            return Err(anyhow!(
                    "ERROR: incorrectly formatted peer info. Should be formatted as: `pubkey@host:port`",
            ));
        }

        let peer_addr = peer_addr_str
            .unwrap()
            .to_socket_addrs()
            .map(|mut r| r.next());
        if peer_addr.is_err() || peer_addr.as_ref().unwrap().is_none() {
            return Err(anyhow!(
                "ERROR: couldn't parse pubkey@host:port into a socket address",
            ));
        }

        let pubkey = hex_utils::to_compressed_pubkey(pubkey.unwrap());
        if pubkey.is_none() {
            return Err(anyhow!("ERROR: unable to parse given pubkey for node",));
        }

        Ok((pubkey.unwrap(), peer_addr.unwrap().unwrap()))
    }
}

#[derive(Debug, Deserialize)]
pub struct BitcoindConfig {
    pub rpc_user: String,
    pub rpc_password_file: PathBuf,
    pub rpc_host: String,
    pub rpc_port: u16,
    pub network: bitcoin::Network,
}
