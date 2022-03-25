use super::flat;
use crate::gossip::{GossipMessage, Message};

#[repr(transparent)]
struct FinishedBytes<'a>(&'a [u8]);

impl<'a> From<(&'a mut flatbuffers::FlatBufferBuilder<'_>, &GossipMessage)> for FinishedBytes<'a> {
    fn from((builder, msg): (&'a mut flatbuffers::FlatBufferBuilder<'_>, &GossipMessage)) -> Self {
        builder.reset();
        match msg.msg {
            Message::NodeAnnouncement { node_id } => {
                let pubkey = flat::PubKey(node_id.serialize());
                let node_announcement = flat::NodeAnnouncement::create(
                    builder,
                    &flat::NodeAnnouncementArgs {
                        node_id: Some(&pubkey),
                    },
                );
                let msg = flat::GossipRecord::create(
                    builder,
                    &flat::GossipRecordArgs {
                        received_at: msg.received_at.into(),
                        msg_type: flat::Message::NodeAnnouncement,
                        msg: Some(node_announcement.as_union_value()),
                    },
                );
                builder.finish(msg, None);
                FinishedBytes(builder.finished_data())
            }
            _ => unimplemented!(),
        }
    }
}
