use futures::Stream;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use crate::config::DEFAULT_CHANNEL_SIZE;

pub trait BusEventFilter<T>: Fn(&T) -> bool + Send + Sync {}
impl<F: Send + Sync, T> BusEventFilter<T> for F where F: Fn(&T) -> bool {}

pub struct MessageBus<T> {
    inbound: mpsc::Sender<T>,
    subscribers: Arc<RwLock<Vec<(Box<dyn BusEventFilter<T>>, mpsc::Sender<T>)>>>,
}

impl<T: Clone + Send + Sync + 'static> MessageBus<T> {
    pub fn new() -> Self {
        let (inbound, mut receiver): (mpsc::Sender<T>, mpsc::Receiver<T>) =
            mpsc::channel(DEFAULT_CHANNEL_SIZE);
        let subscribers: Arc<RwLock<Vec<(Box<dyn BusEventFilter<T>>, mpsc::Sender<T>)>>> =
            Arc::new(RwLock::new(Vec::new()));
        let subs = Arc::clone(&subscribers);
        tokio::spawn(async move {
            let mut to_remove = Vec::new();
            while let Some(event) = receiver.recv().await {
                {
                    let subs = subs.read().await;
                    let len = subs.len();
                    for i in 0..len - 1 {
                        let (filter, inbound) = &subs[i];
                        if filter(&event) {
                            if let Err(_) = inbound.send(event.clone()).await {
                                eprintln!("Error sending event");
                                to_remove.push(i);
                            }
                        }
                    }
                    let (filter, inbound) = &subs[len - 1];
                    if filter(&event) {
                        if let Err(_) = inbound.send(event).await {
                            eprintln!("Error sending event");
                            to_remove.push(len - 1);
                        }
                    }
                }
            }
            if to_remove.len() > 0 {
                let mut subs = subs.write().await;
                for i in to_remove.drain(..) {
                    let _ = subs.remove(i);
                }
            }
        });
        Self {
            inbound,
            subscribers,
        }
    }

    pub fn subscribe(&self, filter: impl BusEventFilter<T> + 'static) -> mpsc::Receiver<T> {
        let (inbound, receiver) = mpsc::channel(DEFAULT_CHANNEL_SIZE);
        let subscribers = Arc::clone(&self.subscribers);
        tokio::spawn(async move {
            let mut subs = subscribers.write().await;
            subs.push((Box::new(filter), inbound));
        });
        receiver
    }

    pub fn register_inbound(&self, mut receiver: mpsc::Receiver<impl Into<T> + Send + 'static>) {
        let inbound = self.inbound.clone();
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                if let Err(_) = inbound.send(event.into()).await {
                    eprintln!("Couldn't forward event");
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Dummy {
        value: u32,
    }

    #[tokio::test]
    async fn test() {
        let bus = MessageBus::new();
        let mut filtered = bus.subscribe(|Dummy { value }: &Dummy| *value > 2);
        let mut unfiltered = bus.subscribe(|_: &Dummy| true);

        let (sender, receive) = mpsc::channel(5);
        bus.register_inbound(receive);

        let _ = sender.send(Dummy { value: 1 }).await;
        let _ = sender.send(Dummy { value: 5 }).await;

        assert_eq!(filtered.recv().await.unwrap().value, 5);
        assert_eq!(unfiltered.recv().await.unwrap().value, 1);
        assert_eq!(unfiltered.recv().await.unwrap().value, 5);
    }
}
