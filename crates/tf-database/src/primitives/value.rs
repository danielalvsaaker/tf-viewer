use crate::Result;
use serde::{de::DeserializeOwned, Serialize};

pub trait Value
where
    Self: Sized,
{
    fn as_bytes(&self) -> Result<Vec<u8>>;
    fn from_bytes(data: &[u8]) -> Result<Self>;
}

impl<T> Value for T
where
    T: Serialize + DeserializeOwned,
{
    fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(pot::to_vec(self)?)
    }

    fn from_bytes(data: &[u8]) -> Result<Self> {
        Ok(pot::from_slice(data)?)
    }
}
