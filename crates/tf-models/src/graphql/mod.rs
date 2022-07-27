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

macro_rules! wrap_unit {
    ($name:ident, $storage_unit:ident, $unit:ident) => {
        wrap_unit!($name, $storage_unit, $unit, $name);
    };

    ($name:ident, $storage_unit:ident, $unit:ident, $custom_name:ident) => {
        pub type $custom_name = Unit<::uom::si::$storage_unit::$name>;
        ::async_graphql::scalar!($custom_name);

        impl $custom_name {
            pub fn new<N>(v: ::uom::si::$storage_unit::V) -> Self
            where
                N: ::uom::si::$unit::Unit
                    + ::uom::Conversion<
                        ::uom::si::$storage_unit::V,
                        T = <::uom::si::$storage_unit::V as ::uom::Conversion<
                            ::uom::si::$storage_unit::V,
                        >>::T,
                    >,
            {
                Self(::uom::si::$storage_unit::$name::new::<N>(v))
            }
        }
    };
}

wrap_unit!(AngularVelocity, f64, angular_velocity);
wrap_unit!(Energy, u32, energy);
wrap_unit!(Length, u32, length, LengthU32);
wrap_unit!(Length, f64, length, LengthF64);
wrap_unit!(Power, u16, power);
wrap_unit!(Velocity, f64, velocity);
pub type Duration = Unit<std::time::Duration>;
pub type DateTime = chrono::DateTime<chrono::Local>;

async_graphql::scalar!(Duration);
async_graphql::scalar!(ActivityId);
async_graphql::scalar!(GearId);
async_graphql::scalar!(UserId);
