use bitcoin::{blockdata::constants, secp256k1::key::PublicKey};
use lightning::{
    ln::msgs::*,
    util::events::{MessageSendEvent, MessageSendEventsProvider},
};
use std::sync::Mutex;
use tokio::sync::mpsc;

use super::message::GossipMessage;

pub struct RoutingMessageForwarder {
    bitcoin_network: bitcoin::Network,
    sender: mpsc::Sender<GossipMessage>,
    pending_events: Mutex<Vec<MessageSendEvent>>,
}

impl RoutingMessageForwarder {
    pub fn new(bitcoin_network: bitcoin::Network, sender: mpsc::Sender<GossipMessage>) -> Self {
        Self {
            bitcoin_network,
            sender,
            pending_events: Mutex::new(Vec::new()),
        }
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
    fn handle_node_announcement(&self, msg: &NodeAnnouncement) -> Result<bool, LightningError> {
        self.forward_message(GossipMessage::NodeAnnouncement {
            node_id: msg.contents.node_id.into(),
        });
        Ok(false)
    }

    fn handle_channel_announcement(
        &self,
        msg: &ChannelAnnouncement,
    ) -> Result<bool, LightningError> {
        self.forward_message(GossipMessage::ChannelAnnouncement {
            short_channel_id: msg.contents.short_channel_id,
            node_a_id: msg.contents.node_id_1.into(),
            node_b_id: msg.contents.node_id_2.into(),
        });
        Ok(false)
    }

    fn handle_channel_update(&self, msg: &ChannelUpdate) -> Result<bool, LightningError> {
        self.forward_message(GossipMessage::ChannelUpdate {
            short_channel_id: msg.contents.short_channel_id,
            timestamp: msg.contents.timestamp,
            cltv_expiry_delta: msg.contents.cltv_expiry_delta,
            htlc_minimum_msat: msg.contents.htlc_minimum_msat.into(),
            htlc_maximum_msat: match msg.contents.htlc_maximum_msat {
                OptionalField::Present(msats) => Some(msats.into()),
                OptionalField::Absent => None,
            },
            fee_base_msat: msg.contents.fee_base_msat.into(),
            fee_proportional_millionths: msg.contents.fee_proportional_millionths,
        });
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

    fn peer_connected(&self, their_node_id: &PublicKey, init_msg: &Init) {
        // We will only perform a sync with peers that support gossip_queries.
        if !init_msg.features.supports_gossip_queries() {
            return ();
        }

        // Send a gossip_timestamp_filter to enable gossip message receipt. Note that we have to
        // use a "all timestamps" filter as sending the current timestamp would result in missing
        // gossip messages that are simply sent late. We could calculate the intended filter time
        // by looking at the current time and subtracting two weeks (before which we'll reject
        // messages), but there's not a lot of reason to bother - our peers should be discarding
        // the same messages.
        let mut pending_events = self.pending_events.lock().unwrap();
        pending_events.push(MessageSendEvent::SendGossipTimestampFilter {
            node_id: their_node_id.clone(),
            msg: GossipTimestampFilter {
                chain_hash: constants::genesis_block(self.bitcoin_network).block_hash(),
                first_timestamp: 0,
                timestamp_range: u32::max_value(),
            },
        });
    }

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
        let mut ret = Vec::new();
        let mut pending_events = self.pending_events.lock().unwrap();
        core::mem::swap(&mut ret, &mut pending_events);
        ret
    }
}
