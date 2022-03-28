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

pub trait BusSubscriber<T>
where
    Self: Sized,
{
    fn filter(event: &T) -> bool;
    fn convert(event: T) -> Option<Self>;
}

impl<T: Clone + Send + Sync + std::fmt::Debug + 'static> MessageBus<T> {
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
                                if let Err(e) = inbound.send(event.clone()).await {
                                    eprintln!("Error forwarding event: {:?}", e);
                                    to_remove.push(i);
                                }
                            }
                        }
                        let (filter, inbound) = &subs[0];
                        if filter(&event) {
                            if let Err(e) = inbound.send(event).await {
                                eprintln!("Error forwarding event: {:?}", e);
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

    pub async fn subscribe<S: BusSubscriber<T> + 'static>(&self) -> impl Stream<Item = S> {
        let (inbound, receiver) = mpsc::channel(self.buffer_size);
        {
            let mut subs = self.subscribers.write().await;
            subs.push((Box::new(<S as BusSubscriber<T>>::filter), inbound));
        }
        ReceiverStream::new(receiver).filter_map(|event: T| <S as BusSubscriber<T>>::convert(event))
    }

    pub async fn dispatch(
        &self,
        msg: impl Into<T> + Send + std::fmt::Debug,
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

    impl BusSubscriber<Dummy> for u8 {
        fn filter(Dummy { value }: &Dummy) -> bool {
            *value <= u8::MAX as u32
        }

        fn convert(Dummy { value }: Dummy) -> Option<u8> {
            Some(value as u8)
        }
    }

    impl BusSubscriber<Dummy> for u32 {
        fn filter(_: &Dummy) -> bool {
            true
        }

        fn convert(Dummy { value }: Dummy) -> Option<u32> {
            Some(value)
        }
    }

    #[tokio::test]
    async fn test() {
        let bus = MessageBus::new(10);
        let mut filtered = bus.subscribe::<u8>().await;
        let mut unfiltered = bus.subscribe::<u32>().await;

        let _ = bus.dispatch(Dummy { value: 1 }).await.unwrap();
        let _ = bus
            .dispatch(Dummy {
                value: u8::MAX as u32 + 1,
            })
            .await
            .unwrap();

        assert_eq!(filtered.next().await.unwrap(), 1);
        assert_eq!(unfiltered.next().await.unwrap(), 1);
        assert_eq!(unfiltered.next().await.unwrap(), u8::MAX as u32 + 1);
    }
}
