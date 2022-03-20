mod forwarder;
mod graph_update;

pub use forwarder::UncertaintyGraphMsgForwarder;

use tokio::sync::mpsc;

pub fn init_uncertainty_graph() -> UncertaintyGraphMsgForwarder {
    let (sender, _receiver) = mpsc::channel(1000);
    UncertaintyGraphMsgForwarder::new(sender)
}

pub struct UncertaintyGraph {}
