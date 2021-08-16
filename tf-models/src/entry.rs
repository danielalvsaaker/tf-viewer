use serde::Serialize;
use crate::error::Result;
use super::UserForm;

pub trait Entry<'a> {
    fn to_value(&self) -> Result<Vec<u8>>;
}

impl<'a, S: Serialize> Entry<'a> for S {
    fn to_value(&self) -> Result<Vec<u8>> {
        Ok(rmp_serde::to_vec(self)?)
    }
}
