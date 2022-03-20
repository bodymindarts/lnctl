mod channel;
mod forwarder;
mod graph;
mod graph_update;
mod node;
mod pool;

pub use channel::UncertaintyChannel;
pub use forwarder::UncertaintyGraphMsgForwarder;
pub use pool::GraphPool;

use tokio::sync::mpsc;

pub fn init_uncertainty_graph() -> (pool::GraphPool, UncertaintyGraphMsgForwarder) {
    let (sender, receiver) = mpsc::channel(10000);
    let pool = pool::GraphPool::new(2, receiver);

    (pool, UncertaintyGraphMsgForwarder::new(sender))
}
