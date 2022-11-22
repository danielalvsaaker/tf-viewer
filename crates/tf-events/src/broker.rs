use crate::{Event, FollowerEvent, Handler};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::watch::{error::SendError, Receiver, Sender};

pub type Topic<T> = Arc<DashMap<<T as Event>::Key, Arc<Sender<<T as Event>::Value>>>>;

#[derive(Clone, Default)]
pub struct Broker {
    pub follower_event: Topic<FollowerEvent>,
}

impl Broker {
    pub fn subscribe<T>(&self, key: T::Key) -> Receiver<T::Value>
    where
        Self: Handler<T>,
        T: Event,
        T::Key: Copy + Send + Sync + 'static,
        T::Value: Send + Sync + 'static,
    {
        let topic = self.handle();

        match topic.get(&key).map(|sender| sender.subscribe()) {
            Some(inner) => inner,
            _ => {
                let (sender, receiver) = tokio::sync::watch::channel(T::Value::default());
                let sender = std::sync::Arc::new(sender);

                topic.insert(key, sender.clone());

                tokio::spawn(async move {
                    sender.closed().await;
                    topic.remove(&key).unwrap();

                    println!("Dropped");
                });

                receiver
            }
        }
    }

    pub fn publish<T>(&self, key: T::Key, value: T::Value) -> Result<(), SendError<T::Value>>
    where
        Self: Handler<T>,
        T: Event,
    {
        if let Some(sender) = self.handle().get(&key) {
            dbg!(sender.receiver_count());
            sender.send(value)
        } else {
            Ok(())
        }
    }
}
