mod sport;
pub use sport::{Sport, SPORTS};
pub mod activity;
pub use activity::Activity;
pub mod gear;
pub mod user;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Id<const L: usize> {
    inner: [u8; L],
}

impl<const L: usize> Id<L> {
    const LENGTH: usize = L;

    pub fn as_bytes(&self) -> [u8; L] {
        self.inner
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.inner).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidLengthError> {
        Ok(Self {
            inner: bytes.try_into().map_err(|_| InvalidLengthError {
                expected: L,
                actual: bytes.len(),
            })?,
        })
    }
}

impl<const L: usize> std::str::FromStr for Id<L> {
    type Err = InvalidLengthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

impl<const L: usize> Serialize for Id<L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer
            .serialize_str(std::str::from_utf8(&self.inner).map_err(serde::ser::Error::custom)?)
    }
}

impl<'de, const L: usize> Deserialize<'de> for Id<L> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::from_bytes(String::deserialize(deserializer)?.as_bytes())
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug)]
pub struct InvalidLengthError {
    expected: usize,
    actual: usize,
}

impl std::fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "invalid id length, expected: {}, actual: {}",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for InvalidLengthError {}

macro_rules! declare_id {
    ($name:ident, $length:expr) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(Id<$length>);

        impl $name {
            pub const LENGTH: usize = Id::<$length>::LENGTH;

            pub fn new() -> Self {
                ::nanoid::nanoid!($length).parse().unwrap()
            }

            pub fn as_bytes(&self) -> [u8; Self::LENGTH] {
                self.0.as_bytes()
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidLengthError> {
                Ok(Self(Id::from_bytes(bytes)?))
            }
        }

        impl ::std::default::Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(::std::str::from_utf8(&self.as_bytes()).unwrap_or_default())
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = InvalidLengthError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_bytes(s.as_bytes())
            }
        }
    };
}

declare_id!(UserId, 21);
declare_id!(GearId, 21);
declare_id!(ActivityId, 12);
declare_id!(ClientId, 21);

#[cfg(feature = "graphql")]
#[path = "graphql/mod.rs"]
pub mod types;

#[cfg(not(feature = "graphql"))]
pub mod types {
    pub use std::time::Duration;
    pub use uom::si::{
        f64::{AngularVelocity, Length as LengthF64, Velocity},
        u16::Power,
        u32::{Energy, Length as LengthU32},
    };
}
