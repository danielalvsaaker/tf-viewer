use crate::primitives::{Key, Value};

pub mod activity;
pub mod gear;
pub mod user;

pub mod index;

pub trait Resource: Value {
    const NAME: &'static str;

    type Key: Key;
}
