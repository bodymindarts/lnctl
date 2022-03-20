use crate::{node::logger::LnCtlLogger, uncertainty_graph::UncertaintyGraphMsgForwarder};
use bitcoin::secp256k1::key::PublicKey;
use lightning::{
    chain,
    ln::msgs::*,
    routing::network_graph::{NetGraphMsgHandler, NetworkGraph, NodeId},
    util::events::{MessageSendEvent, MessageSendEventsProvider},
};
use std::sync::Arc;

pub(crate) type ArcNetGraphMsgHandler = Arc<
    NetGraphMsgHandler<Arc<NetworkGraph>, Arc<dyn chain::Access + Send + Sync>, Arc<LnCtlLogger>>,
>;

pub struct MessageForwarder {
    inner: ArcNetGraphMsgHandler,
    forwarder: UncertaintyGraphMsgForwarder,
}

impl MessageForwarder {
    pub fn new(inner: ArcNetGraphMsgHandler, forwarder: UncertaintyGraphMsgForwarder) -> Self {
        MessageForwarder { inner, forwarder }
    }
}

impl RoutingMessageHandler for MessageForwarder {
    fn handle_node_announcement(&self, msg: &NodeAnnouncement) -> Result<bool, LightningError> {
        let res = self.inner.handle_node_announcement(msg)?;
        let node_id = NodeId::from_pubkey(&msg.contents.node_id);
        if let Some(node) = self.inner.network_graph().read_only().nodes().get(&node_id) {
            self.forwarder.update_node(node_id, node);
        }
        Ok(res)
    }
    fn handle_channel_announcement(
        &self,
        msg: &ChannelAnnouncement,
    ) -> Result<bool, LightningError> {
        let res = self.inner.handle_channel_announcement(msg)?;
        let channel_id = msg.contents.short_channel_id;
        if let Some(channel) = self
            .inner
            .network_graph()
            .read_only()
            .channels()
            .get(&channel_id)
        {
            self.forwarder.update_channel(channel_id, channel);
        } else {
            self.forwarder.remove_channel(channel_id);
        }
        Ok(res)
    }
    fn handle_channel_update(&self, msg: &ChannelUpdate) -> Result<bool, LightningError> {
        let res = self.inner.handle_channel_update(msg)?;
        let channel_id = msg.contents.short_channel_id;
        if let Some(channel) = self
            .inner
            .network_graph()
            .read_only()
            .channels()
            .get(&channel_id)
        {
            self.forwarder.update_channel(channel_id, channel);
        } else {
            self.forwarder.remove_channel(channel_id);
        }
        Ok(res)
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
        return self
            .inner
            .get_next_channel_announcements(starting_point, batch_amount);
    }
    fn get_next_node_announcements(
        &self,
        starting_point: Option<&PublicKey>,
        batch_amount: u8,
    ) -> Vec<NodeAnnouncement> {
        return self
            .inner
            .get_next_node_announcements(starting_point, batch_amount);
    }
    fn sync_routing_table(&self, their_node_id: &PublicKey, init: &Init) {
        return self.inner.sync_routing_table(their_node_id, init);
    }
    fn handle_reply_channel_range(
        &self,
        their_node_id: &PublicKey,
        msg: ReplyChannelRange,
    ) -> Result<(), LightningError> {
        return self.inner.handle_reply_channel_range(their_node_id, msg);
    }
    fn handle_reply_short_channel_ids_end(
        &self,
        their_node_id: &PublicKey,
        msg: ReplyShortChannelIdsEnd,
    ) -> Result<(), LightningError> {
        return self
            .inner
            .handle_reply_short_channel_ids_end(their_node_id, msg);
    }
    fn handle_query_channel_range(
        &self,
        their_node_id: &PublicKey,
        msg: QueryChannelRange,
    ) -> Result<(), LightningError> {
        return self.inner.handle_query_channel_range(their_node_id, msg);
    }
    fn handle_query_short_channel_ids(
        &self,
        their_node_id: &PublicKey,
        msg: QueryShortChannelIds,
    ) -> Result<(), LightningError> {
        return self
            .inner
            .handle_query_short_channel_ids(their_node_id, msg);
    }
}

impl MessageSendEventsProvider for MessageForwarder {
    fn get_and_clear_pending_msg_events(&self) -> Vec<MessageSendEvent> {
        return self.inner.get_and_clear_pending_msg_events();
    }
}
