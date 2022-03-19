use crate::Result;
use serde::{de::DeserializeOwned, Serialize};

pub trait Value
where
    Self: Sized + Serialize + DeserializeOwned,
{
    fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(pot::to_vec(self)?)
    }

    fn from_bytes(data: &[u8]) -> Result<Self> {
        Ok(pot::from_slice(data)?)
    }
}

impl<T> Value for T where T: Sized + Serialize + DeserializeOwned {}
