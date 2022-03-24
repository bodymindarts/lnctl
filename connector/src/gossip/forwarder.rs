use bitcoin::secp256k1::key::PublicKey;
use lightning::{
    ln::msgs::*,
    util::events::{MessageSendEvent, MessageSendEventsProvider},
};
use tokio::sync::mpsc;

use super::message::GossipMessage;

pub struct RoutingMessageForwarder {
    sender: mpsc::Sender<GossipMessage>,
}

impl RoutingMessageForwarder {
    pub fn new(sender: mpsc::Sender<GossipMessage>) -> Self {
        Self { sender }
    }

    fn forward_message(&self, msg: GossipMessage) {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            if let Err(e) = sender.send(msg).await {
                eprintln!("Error forawding msg: {}", e);
            }
        });
    }
}

impl RoutingMessageHandler for RoutingMessageForwarder {
    fn handle_node_announcement(&self, _msg: &NodeAnnouncement) -> Result<bool, LightningError> {
        self.forward_message(GossipMessage::NodeAnnouncement {});
        Ok(false)
    }
    fn handle_channel_announcement(
        &self,
        _msg: &ChannelAnnouncement,
    ) -> Result<bool, LightningError> {
        Ok(false)
    }
    fn handle_channel_update(&self, _msg: &ChannelUpdate) -> Result<bool, LightningError> {
        Ok(false)
    }

    fn get_next_channel_announcements(
        &self,
        _starting_point: u64,
        _batch_amount: u8,
    ) -> Vec<(
        ChannelAnnouncement,
        Option<ChannelUpdate>,
        Option<ChannelUpdate>,
    )> {
        Vec::new()
    }
    fn get_next_node_announcements(
        &self,
        _starting_point: Option<&PublicKey>,
        _batch_amount: u8,
    ) -> Vec<NodeAnnouncement> {
        Vec::new()
    }
    fn sync_routing_table(&self, _their_node_id: &PublicKey, _init: &Init) {}

    fn handle_reply_channel_range(
        &self,
        _their_node_id: &PublicKey,
        _msg: ReplyChannelRange,
    ) -> Result<(), LightningError> {
        Ok(())
    }
    fn handle_reply_short_channel_ids_end(
        &self,
        _their_node_id: &PublicKey,
        _msg: ReplyShortChannelIdsEnd,
    ) -> Result<(), LightningError> {
        Ok(())
    }
    fn handle_query_channel_range(
        &self,
        _their_node_id: &PublicKey,
        _msg: QueryChannelRange,
    ) -> Result<(), LightningError> {
        Ok(())
    }
    fn handle_query_short_channel_ids(
        &self,
        _their_node_id: &PublicKey,
        _msg: QueryShortChannelIds,
    ) -> Result<(), LightningError> {
        Ok(())
    }
}

impl MessageSendEventsProvider for RoutingMessageForwarder {
    fn get_and_clear_pending_msg_events(&self) -> Vec<MessageSendEvent> {
        return Vec::new();
    }
}
