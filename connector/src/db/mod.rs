pub mod flat {
    pub use crate::shared_generated::*;
    pub mod gossip {
        use super::*;
        include!("../../../flatbuffers/gen/connector/gossip_generated.rs");
    }
    pub mod channels {
        use super::*;
        include!("../../../flatbuffers/gen/connector/channels_scrape_generated.rs");
    }
}

mod convert;
mod keys;

use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use zerocopy::*;

use crate::{bus::*, config};
use convert::FinishedBytes;
use keys::*;

pub(crate) struct Db {
    _inner: sled::Db,
    gossip: sled::Tree,
    channels: sled::Tree,
}

impl Db {
    pub fn new(data_dir: &PathBuf, bus: ConnectorBus) -> anyhow::Result<Self> {
        let db: sled::Db = sled::open(format!("{}/sled", data_dir.display()))?;
        let gossip = db.open_tree("gossip")?;
        Self::persist_gossip(gossip.clone(), bus.clone());
        let channels = db.open_tree("channels")?;
        Self::persist_channels(channels.clone(), bus);
        Ok(Self {
            _inner: db,
            gossip,
            channels,
        })
    }

    fn persist_gossip(gossip_db: sled::Tree, bus: ConnectorBus) {
        tokio::spawn(async move {
            let mut stream = bus.subscribe::<LdkGossip>().await;
            let mut buffer = flatbuffers::FlatBufferBuilder::new();
            while let Some(msg) = stream.next().await {
                let key = GossipMessageKey::from(&msg);
                let finished_bytes = FinishedBytes::from((&mut buffer, msg));
                if let Err(e) = gossip_db.insert(key.as_bytes(), *finished_bytes) {
                    eprintln!("Couldn't persist gossip: {}", e);
                }
            }
        });
    }

    fn persist_channels(channels_db: sled::Tree, bus: ConnectorBus) {
        tokio::spawn(async move {
            let mut stream = bus.subscribe::<ChannelScrape>().await;
            let mut buffer = flatbuffers::FlatBufferBuilder::new();
            while let Some(scrape) = stream.next().await {
                let mut updated = false;
                let key = ChannelStateKey::from(&scrape);
                let finished_bytes = FinishedBytes::from((&mut buffer, scrape.clone()));
                if let Err(e) = channels_db.update_and_fetch(key.as_bytes(), |existing| {
                    if let Some(existing) = existing {
                        if let Ok(record) = flat::channels::root_as_channel_scrape(&existing) {
                            let new_state =
                                flat::channels::root_as_channel_scrape(&finished_bytes).unwrap();
                            if !channel_state_eq(record, new_state) {
                                updated = true;
                            }
                        }
                    } else {
                        updated = true;
                    }
                    Some(*finished_bytes)
                }) {
                    eprintln!("Couldn't persist channel: {}", e);
                }
                if updated {
                    if let Err(e) = bus.dispatch(MonitoredChannelUpdate { scrape }).await {
                        eprintln!("Couldn't dispatch update: {}", e);
                    }
                }
            }
        });
    }

    pub fn load_gossip<T>(&self) -> impl Stream<Item = T>
    where
        for<'a> T: TryFrom<flat::gossip::GossipRecord<'a>> + Send + Sync + 'static,
    {
        let mut iter = self.gossip.iter();
        let (sender, receiver) = mpsc::channel(config::DEFAULT_CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(Ok((_, value))) = iter.next() {
                let bytes = value.as_ref();
                if let Ok(record) = flat::gossip::root_as_gossip_record(bytes) {
                    let mut to_send = T::try_from(record).ok();
                    if let Some(msg) = to_send.take() {
                        if let Err(e) = sender.send(msg).await {
                            eprintln!("Couldn't send loaded gossip: {}", e);
                        }
                    }
                }
            }
        });
        ReceiverStream::new(receiver)
    }

    pub fn load_channels<T>(&self) -> impl Stream<Item = T>
    where
        for<'a> T: TryFrom<flat::channels::ChannelScrape<'a>> + Send + Sync + 'static,
    {
        let mut iter = self.channels.iter();
        let (sender, receiver) = mpsc::channel(config::DEFAULT_CHANNEL_SIZE);
        tokio::spawn(async move {
            while let Some(Ok((_, value))) = iter.next() {
                let bytes = value.as_ref();
                if let Ok(record) = flat::channels::root_as_channel_scrape(bytes) {
                    let mut to_send = T::try_from(record).ok();
                    if let Some(msg) = to_send.take() {
                        if let Err(e) = sender.send(msg).await {
                            eprintln!("Couldn't send loaded gossip: {}", e);
                        }
                    }
                }
            }
        });
        ReceiverStream::new(receiver)
    }
}

fn channel_state_eq<'a, 'b>(
    a: flat::channels::ChannelScrape<'a>,
    b: flat::channels::ChannelScrape<'b>,
) -> bool {
    if let (Some(a_state), Some(b_state)) = (a.state(), b.state()) {
        let a_table = a_state._tab;
        let b_table = b_state._tab;
        a_table.buf[a_table.loc..].eq(&b_table.buf[b_table.loc..])
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::primitives::*;

    #[test]
    fn test_channel_state_equality() {
        let mut scrape = ChannelScrape {
            scraped_at: UnixTimestampSecs::now(),
            state: ChannelState {
                short_channel_id: 0,
                local_node_id: "037037c24107b9b6727361c5e99039465078e79ec5913556d55eeaeabc10142532"
                    .parse::<MonitoredNodeId>()
                    .unwrap(),
                remote_node_id:
                    "037037c24107b9b6727361c5e99039465078e79ec5913556d55eeaeabc10142533"
                        .parse::<NodeId>()
                        .unwrap(),
                active: true,
                private: true,
                capacity: Satoshi::from(3_u64),
                local_balance: Satoshi::from(1_u64),
                remote_balance: Satoshi::from(1_u64),
                unsettled_balance: Satoshi::from(1_u64),
                local_channel_settings: ChannelSettings {
                    chan_reserve_sat: Satoshi::from(1_u64),
                    htlc_minimum_msat: MilliSatoshi::from(1_u64),
                },
                remote_channel_settings: ChannelSettings {
                    chan_reserve_sat: Satoshi::from(1_u64),
                    htlc_minimum_msat: MilliSatoshi::from(1_u64),
                },
            },
        };
        let mut original_buffer = flatbuffers::FlatBufferBuilder::new();
        let original_bytes = FinishedBytes::from((&mut original_buffer, scrape.clone()));
        let original_record = flat::channels::root_as_channel_scrape(&original_bytes).unwrap();

        scrape.scraped_at = UnixTimestampSecs::from(1_u64);
        let mut new_scrape_buffer = flatbuffers::FlatBufferBuilder::new();
        let new_scrape_bytes = FinishedBytes::from((&mut new_scrape_buffer, scrape.clone()));
        let new_scrape = flat::channels::root_as_channel_scrape(&new_scrape_bytes).unwrap();

        scrape.state.capacity = Satoshi::from(u64::MAX);
        let mut new_state_buffer = flatbuffers::FlatBufferBuilder::new();
        let new_state_bytes = FinishedBytes::from((&mut new_state_buffer, scrape.clone()));
        let new_state = flat::channels::root_as_channel_scrape(&new_state_bytes).unwrap();

        assert!(channel_state_eq(original_record, new_scrape));
        assert!(!channel_state_eq(original_record, new_state));

        // To ensure backwards compatibility, this comparison should continue to work. (Though its
        // not terrible if it doesn't...).
        let hard_coded_buffer = [
            16, 0, 0, 0, 0, 0, 0, 0, 8, 0, 22, 0, 8, 0, 4, 0, 8, 0, 0, 0, 44, 0, 0, 0, 149, 167,
            69, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 118, 0, 0, 0, 6, 0, 39, 0, 0, 0, 5, 0, 80,
            0, 88, 0, 96, 0, 104, 0, 72, 0, 76, 0, 26, 0, 0, 0, 0, 1, 3, 112, 55, 194, 65, 7, 185,
            182, 114, 115, 97, 197, 233, 144, 57, 70, 80, 120, 231, 158, 197, 145, 53, 86, 213, 94,
            234, 234, 188, 16, 20, 37, 50, 3, 112, 55, 194, 65, 7, 185, 182, 114, 115, 97, 197,
            233, 144, 57, 70, 80, 120, 231, 158, 197, 145, 53, 86, 213, 94, 234, 234, 188, 16, 20,
            37, 51, 76, 0, 0, 0, 48, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 18, 0, 4, 0, 6, 0, 0,
            0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 12, 0, 4, 0, 6, 0, 0, 0, 1, 0, 0, 0,
            0, 0, 0, 0,
        ];
        let hard_coded = flat::channels::root_as_channel_scrape(&hard_coded_buffer).unwrap();
        assert!(channel_state_eq(original_record, hard_coded));
    }
}
