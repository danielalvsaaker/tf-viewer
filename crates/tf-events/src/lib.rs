use std::hash::Hash;
use tf_models::UserId;

mod broker;

pub use broker::{Broker, Topic};

pub trait Handler<T>
where
    T: Event,
{
    fn handle(&self) -> Topic<T>;
}

pub trait Event {
    type Key: Hash + Eq;
    type Value: Default;
}

pub struct FollowerEvent;

impl Event for FollowerEvent {
    type Key = UserId;
    type Value = ();
}

impl Handler<FollowerEvent> for Broker {
    fn handle(&self) -> Topic<FollowerEvent> {
        self.follower_event.clone()
    }
}
