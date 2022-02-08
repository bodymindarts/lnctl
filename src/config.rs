use anyhow::*;
use lightning::ln::msgs::NetAddress;
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "bitcoind")]
    pub bitcoind_config: BitcoindConfig,
    pub data_dir: PathBuf,
    pub listen_port: u16,
    pub public_address: Option<SocketAddr>,
    #[serde(skip_deserializing)]
    pub net_address: Option<NetAddress>,
    pub announced_node_name: [u8; 32],
}

impl Config {
    pub fn from_path(path: PathBuf) -> Result<Self, anyhow::Error> {
        let config_file = std::fs::read_to_string(path).context("Couldn't read config file")?;
        let mut config: Config =
            serde_yaml::from_str(&config_file).context("Couldn't parse config file")?;
        config.init_net_address();
        Ok(config)
    }

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
}

#[derive(Debug, Deserialize)]
pub struct BitcoindConfig {
    pub rpc_user: String,
    pub rpc_password_file: PathBuf,
    pub rpc_host: String,
    pub rpc_port: u16,
    pub network: bitcoin::Network,
}
