use crate::node::logger::LnCtlLogger;
use bitcoin::secp256k1::key::PublicKey;
use lightning::{
    chain,
    ln::msgs::*,
    routing::network_graph::NetGraphMsgHandler,
    routing::network_graph::NetworkGraph,
    util::events::{MessageSendEvent, MessageSendEventsProvider},
};
use std::sync::Arc;

pub(crate) type ArcNetGraphMsgHandler = Arc<
    NetGraphMsgHandler<Arc<NetworkGraph>, Arc<dyn chain::Access + Send + Sync>, Arc<LnCtlLogger>>,
>;

pub struct MessageForwarder {
    pub inner: ArcNetGraphMsgHandler,
}

impl MessageForwarder {
    pub fn new(
        network_graph: Arc<NetworkGraph>,
        chain_access: Option<Arc<dyn chain::Access + Send + Sync>>,
        logger: Arc<LnCtlLogger>,
    ) -> Self {
        let inner = Arc::new(NetGraphMsgHandler::new(network_graph, chain_access, logger));
        MessageForwarder { inner }
    }
}

impl RoutingMessageHandler for MessageForwarder {
    fn handle_node_announcement(&self, msg: &NodeAnnouncement) -> Result<bool, LightningError> {
        println!("handle_node_announcement: {:?}", msg);
        self.inner.handle_node_announcement(msg)
    }
    fn handle_channel_announcement(
        &self,
        msg: &ChannelAnnouncement,
    ) -> Result<bool, LightningError> {
        println!("handle_channel_announcement: {:?}", msg);
        return self.inner.handle_channel_announcement(msg);
    }
    fn handle_channel_update(&self, msg: &ChannelUpdate) -> Result<bool, LightningError> {
        println!("handle_channel_update: {:?}", msg);
        return self.inner.handle_channel_update(msg);
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
        println!(
            "get_next_channel_announcements: {:?} {:?}",
            starting_point, batch_amount
        );
        return self
            .inner
            .get_next_channel_announcements(starting_point, batch_amount);
    }
    fn get_next_node_announcements(
        &self,
        starting_point: Option<&PublicKey>,
        batch_amount: u8,
    ) -> Vec<NodeAnnouncement> {
        println!(
            "get_next_node_announcements: {:?} {:?}",
            starting_point, batch_amount
        );
        return self
            .inner
            .get_next_node_announcements(starting_point, batch_amount);
    }
    fn sync_routing_table(&self, their_node_id: &PublicKey, init: &Init) {
        println!("sync_routing_table: {:?} {:?}", their_node_id, init);
        return self.inner.sync_routing_table(their_node_id, init);
    }
    fn handle_reply_channel_range(
        &self,
        their_node_id: &PublicKey,
        msg: ReplyChannelRange,
    ) -> Result<(), LightningError> {
        println!("handle_reply_channel_range: {:?} {:?}", their_node_id, msg);
        return self.inner.handle_reply_channel_range(their_node_id, msg);
    }
    fn handle_reply_short_channel_ids_end(
        &self,
        their_node_id: &PublicKey,
        msg: ReplyShortChannelIdsEnd,
    ) -> Result<(), LightningError> {
        println!(
            "handle_reply_short_channel_ids_end: {:?} {:?}",
            their_node_id, msg
        );
        return self
            .inner
            .handle_reply_short_channel_ids_end(their_node_id, msg);
    }
    fn handle_query_channel_range(
        &self,
        their_node_id: &PublicKey,
        msg: QueryChannelRange,
    ) -> Result<(), LightningError> {
        println!("handle_query_channel_range: {:?} {:?}", their_node_id, msg);
        return self.inner.handle_query_channel_range(their_node_id, msg);
    }
    fn handle_query_short_channel_ids(
        &self,
        their_node_id: &PublicKey,
        msg: QueryShortChannelIds,
    ) -> Result<(), LightningError> {
        println!(
            "handle_query_short_channel_ids: {:?} {:?}",
            their_node_id, msg
        );
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
