use zerocopy::*;

use crate::bus::ConnectorMsgSub;
use crate::connector::proto;

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
pub struct ChannelArchiveKey {
    node_id: [u8; 33],
    short_channel_id: U64<BigEndian>,
    scrape_time: U64<BigEndian>,
}

impl TryFrom<&ConnectorMsgSub<proto::MonitoredChannelUpdate>> for ChannelArchiveKey {
    type Error = ();

    fn try_from(
        ConnectorMsgSub { msg, .. }: &ConnectorMsgSub<proto::MonitoredChannelUpdate>,
    ) -> Result<Self, Self::Error> {
        if let Some(proto::node_event::Event::ChannelUpdate(update)) = &msg.node_event.event {
            Ok(ChannelArchiveKey {
                node_id: msg.monitored_node_id.serialize(),
                short_channel_id: U64::new(update.channel_state.as_ref().unwrap().short_channel_id),
                scrape_time: U64::new(update.timestamp),
            })
        } else {
            Err(())
        }
    }
}
