use futures::Stream;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

pub trait BusEventFilter<T>: Fn(&T) -> bool + Send + Sync {}
impl<F: Send + Sync, T> BusEventFilter<T> for F where F: Fn(&T) -> bool {}

#[derive(Clone)]
pub struct MessageBus<T: Clone + Send + Sync + 'static> {
    buffer_size: usize,
    inbound: mpsc::Sender<T>,
    subscribers: Arc<RwLock<Vec<(Box<dyn BusEventFilter<T>>, mpsc::Sender<T>)>>>,
}

impl<T: Clone + Send + Sync + 'static> MessageBus<T> {
    pub fn new(buffer_size: usize) -> Self {
        let (inbound, mut receiver): (mpsc::Sender<T>, mpsc::Receiver<T>) =
            mpsc::channel(buffer_size);
        let subscribers: Arc<RwLock<Vec<(Box<dyn BusEventFilter<T>>, mpsc::Sender<T>)>>> =
            Arc::new(RwLock::new(Vec::new()));
        let subs = Arc::clone(&subscribers);
        tokio::spawn(async move {
            let mut to_remove = Vec::new();
            while let Some(event) = receiver.recv().await {
                {
                    let subs = subs.read().await;
                    let len = subs.len();
                    if len > 0 {
                        for i in 1..len {
                            let (filter, inbound) = &subs[i];
                            if filter(&event) {
                                if let Err(_) = inbound.send(event.clone()).await {
                                    eprintln!("Error sending event");
                                    to_remove.push(i);
                                }
                            }
                        }
                        let (filter, inbound) = &subs[0];
                        if filter(&event) {
                            if let Err(_) = inbound.send(event).await {
                                eprintln!("Error sending event");
                                to_remove.push(0);
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
            }
        });
        Self {
            buffer_size,
            inbound,
            subscribers,
        }
    }

    pub async fn subscribe_with_filter(
        &self,
        filter: impl BusEventFilter<T> + 'static,
    ) -> impl Stream<Item = T> {
        let (inbound, receiver) = mpsc::channel(self.buffer_size);
        {
            let mut subs = self.subscribers.write().await;
            subs.push((Box::new(filter), inbound));
        }
        ReceiverStream::new(receiver)
    }

    pub async fn dispatch(
        &self,
        msg: impl Into<T> + Send,
    ) -> Result<(), mpsc::error::SendError<T>> {
        self.inbound.send(msg.into()).await
    }

    pub fn spawn_dispatch(&self, msg: impl Into<T> + Send)
    where
        T: std::fmt::Debug,
    {
        let inbound = self.inbound.clone();
        let msg = msg.into();
        tokio::spawn(async move {
            if let Err(e) = inbound.send(msg).await {
                eprintln!("Error spawning dispatch: {:?}", e);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct Dummy {
        value: u32,
    }

    #[tokio::test]
    async fn test() {
        let bus = MessageBus::new(10);
        let mut filtered = bus
            .subscribe_with_filter(|Dummy { value }: &Dummy| *value > 2)
            .await;
        let mut unfiltered = bus.subscribe_with_filter(|_: &Dummy| true).await;

        let _ = bus.dispatch(Dummy { value: 1 }).await.unwrap();
        let _ = bus.dispatch(Dummy { value: 5 }).await.unwrap();

        assert_eq!(filtered.next().await.unwrap().value, 5);
        assert_eq!(unfiltered.next().await.unwrap().value, 1);
        assert_eq!(unfiltered.next().await.unwrap().value, 5);
    }
}
