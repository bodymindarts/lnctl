use crate::{bitcoind::BitcoindClient, chain_monitor::ChainMonitor, logger::LnCtlLogger};
use bitcoin::hash_types::BlockHash;
use lightning::{
    chain::{
        self,
        channelmonitor::ChannelMonitor,
        keysinterface::{InMemorySigner, KeysManager},
        Watch,
    },
    ln::channelmanager::{self, ChainParameters, ChannelManagerReadArgs, SimpleArcChannelManager},
    util::{config::UserConfig, ser::ReadableArgs},
};
use lightning_block_sync::{init, poll, UnboundedCache};
use std::{fs, ops::Deref, path::PathBuf, sync::Arc};

pub(crate) type ChannelManager =
    SimpleArcChannelManager<ChainMonitor, BitcoindClient, BitcoindClient, LnCtlLogger>;

pub async fn init_channel_manager(
    data_dir: &PathBuf,
    mut channel_monitors: Vec<(BlockHash, ChannelMonitor<InMemorySigner>)>,
    bitcoind_client: Arc<BitcoindClient>,
    keys_manager: Arc<KeysManager>,
    chain_monitor: Arc<ChainMonitor>,
    cache: &mut UnboundedCache,
    logger: Arc<LnCtlLogger>,
) -> Result<(Arc<ChannelManager>, poll::ValidatedBlockHeader), anyhow::Error> {
    let user_config = get_user_config();
    let mut restarting_node = true;
    let (channel_manager_blockhash, mut channel_manager) = {
        if let Ok(mut f) = fs::File::open(format!("{}/manager", data_dir.as_path().display())) {
            let mut channel_monitor_mut_references = Vec::new();
            for (_, channel_monitor) in channel_monitors.iter_mut() {
                channel_monitor_mut_references.push(channel_monitor);
            }
            let read_args = ChannelManagerReadArgs::new(
                keys_manager,
                Arc::clone(&bitcoind_client),
                Arc::clone(&chain_monitor),
                Arc::clone(&bitcoind_client),
                Arc::clone(&logger),
                user_config,
                channel_monitor_mut_references,
            );
            <(BlockHash, ChannelManager)>::read(&mut f, read_args).unwrap()
        } else {
            // We're starting a fresh node.
            restarting_node = false;

            let best_block = bitcoind_client.get_best_block().await?;
            let latest_blockhash = best_block.block_hash();
            let chain_params = ChainParameters {
                network: bitcoind_client.network,
                best_block,
            };
            let fresh_channel_manager = channelmanager::ChannelManager::new(
                Arc::clone(&bitcoind_client),
                chain_monitor.clone(),
                Arc::clone(&bitcoind_client),
                Arc::clone(&logger),
                keys_manager,
                user_config,
                chain_params,
            );
            (latest_blockhash, fresh_channel_manager)
        }
    };

    let mut chain_listener_channel_monitors = Vec::new();
    let chain_tip = if restarting_node {
        let mut chain_listeners = vec![(
            channel_manager_blockhash,
            &mut channel_manager as &mut dyn chain::Listen,
        )];

        for (blockhash, channel_monitor) in channel_monitors.drain(..) {
            let outpoint = channel_monitor.get_funding_txo().0;
            chain_listener_channel_monitors.push((
                blockhash,
                (
                    channel_monitor,
                    Arc::clone(&bitcoind_client),
                    Arc::clone(&bitcoind_client),
                    Arc::clone(&logger),
                ),
                outpoint,
            ));
        }

        for monitor_listener_info in chain_listener_channel_monitors.iter_mut() {
            chain_listeners.push((
                monitor_listener_info.0,
                &mut monitor_listener_info.1 as &mut dyn chain::Listen,
            ));
        }
        init::synchronize_listeners(
            &mut bitcoind_client.deref(),
            bitcoind_client.network,
            cache,
            chain_listeners,
        )
        .await
        .expect("Couldn't synchronize chain listeners")
    } else {
        init::validate_best_block_header(&mut bitcoind_client.deref())
            .await
            .expect("Couldn't validate best block header")
    };

    for item in chain_listener_channel_monitors.drain(..) {
        let channel_monitor = item.1 .0;
        let funding_outpoint = item.2;
        chain_monitor
            .watch_channel(funding_outpoint, channel_monitor)
            .unwrap();
    }

    Ok((Arc::new(channel_manager), chain_tip))
}

fn get_user_config() -> UserConfig {
    let mut user_config = UserConfig::default();
    user_config
        .peer_channel_config_limits
        .force_announced_channel_preference = false;
    user_config.channel_options.forwarding_fee_base_msat = 0;
    user_config
}
