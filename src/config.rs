use anyhow::*;
use lightning::ln::msgs::NetAddress;
use serde::Deserialize;
use std::{
    env,
    net::{IpAddr, SocketAddr},
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
        Ok(config)
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
