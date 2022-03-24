use bitcoin::secp256k1::key::PublicKey;
use lightning::{
    ln::msgs::*,
    util::events::{MessageSendEvent, MessageSendEventsProvider},
};

pub struct RoutingMessageForwarder {}

impl RoutingMessageForwarder {
    pub fn new() -> Self {
        Self {}
    }
}

impl RoutingMessageHandler for RoutingMessageForwarder {
    fn handle_node_announcement(&self, msg: &NodeAnnouncement) -> Result<bool, LightningError> {
        Ok(false)
    }
    fn handle_channel_announcement(
        &self,
        msg: &ChannelAnnouncement,
    ) -> Result<bool, LightningError> {
        Ok(false)
    }
    fn handle_channel_update(&self, msg: &ChannelUpdate) -> Result<bool, LightningError> {
        Ok(false)
    }

    fn get_next_channel_announcements(
        &self,
        starting_point: u64,
        batch_amount: u8,
    ) -> Vec<(
        ChannelAnnouncement,
        Option<ChannelUpdate>,
        Option<ChannelUpdate>,
    )> {
        Vec::new()
    }
    fn get_next_node_announcements(
        &self,
        starting_point: Option<&PublicKey>,
        batch_amount: u8,
    ) -> Vec<NodeAnnouncement> {
        Vec::new()
    }
    fn sync_routing_table(&self, their_node_id: &PublicKey, init: &Init) {}

    fn handle_reply_channel_range(
        &self,
        their_node_id: &PublicKey,
        msg: ReplyChannelRange,
    ) -> Result<(), LightningError> {
        Ok(())
    }
    fn handle_reply_short_channel_ids_end(
        &self,
        their_node_id: &PublicKey,
        msg: ReplyShortChannelIdsEnd,
    ) -> Result<(), LightningError> {
        Ok(())
    }
    fn handle_query_channel_range(
        &self,
        their_node_id: &PublicKey,
        msg: QueryChannelRange,
    ) -> Result<(), LightningError> {
        Ok(())
    }
    fn handle_query_short_channel_ids(
        &self,
        their_node_id: &PublicKey,
        msg: QueryShortChannelIds,
    ) -> Result<(), LightningError> {
        Ok(())
    }
}

impl MessageSendEventsProvider for RoutingMessageForwarder {
    fn get_and_clear_pending_msg_events(&self) -> Vec<MessageSendEvent> {
        return Vec::new();
    }
}
impl std::ops::Deref for RoutingMessageForwarder {
    type Target = RoutingMessageForwarder;
    fn deref(&self) -> &Self {
        self
    }
}
