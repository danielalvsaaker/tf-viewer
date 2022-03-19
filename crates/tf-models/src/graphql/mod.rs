use crate::{ActivityId, GearId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Unit<T>(T);

impl<T> From<T> for Unit<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> AsRef<T> for Unit<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

pub type Velocity = Unit<uom::si::f64::Velocity>;
pub type Power = Unit<uom::si::u16::Power>;
pub type LengthU32 = Unit<uom::si::u32::Length>;
pub type LengthF64 = Unit<uom::si::f64::Length>;
pub type Duration = Unit<std::time::Duration>;

async_graphql::scalar!(Velocity);
async_graphql::scalar!(Power);
async_graphql::scalar!(LengthU32);
async_graphql::scalar!(LengthF64);
async_graphql::scalar!(Duration);
async_graphql::scalar!(ActivityId);
async_graphql::scalar!(GearId);
async_graphql::scalar!(UserId);
