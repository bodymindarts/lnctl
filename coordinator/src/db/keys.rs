use zerocopy::*;

use crate::bus::ConnectorMsgSub;
use ::shared::proto;

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
pub struct ChannelArchiveKey {
    short_channel_id: U64<BigEndian>,
    scrape_time: U64<BigEndian>,
}

impl ChannelArchiveKey {
    pub fn range(short_channel_id: u64) -> (Self, Self) {
        let start = ChannelArchiveKey {
            short_channel_id: U64::new(short_channel_id),
            scrape_time: U64::new(0),
        };
        let end = ChannelArchiveKey {
            short_channel_id: U64::new(short_channel_id),
            scrape_time: U64::new(u64::MAX),
        };
        (start, end)
    }
}

impl TryFrom<&ConnectorMsgSub<proto::MonitoredChannelUpdate>> for ChannelArchiveKey {
    type Error = ();

    fn try_from(
        ConnectorMsgSub { msg, .. }: &ConnectorMsgSub<proto::MonitoredChannelUpdate>,
    ) -> Result<Self, Self::Error> {
        if let Some(proto::node_event::Event::ChannelUpdate(update)) = &msg.node_event.event {
            Ok(ChannelArchiveKey {
                short_channel_id: U64::new(update.channel_state.as_ref().unwrap().short_channel_id),
                scrape_time: U64::new(update.timestamp),
            })
        } else {
            Err(())
        }
    }
}
