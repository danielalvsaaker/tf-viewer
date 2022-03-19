use crate::{error::Result, Key};
use serde::Deserialize;
use tf_models::UserId;

#[derive(Clone, Copy, Deserialize)]
pub struct UserQuery {
    pub user_id: UserId,
}

impl Key for UserQuery {
    fn as_key(&self) -> Vec<u8> {
        self.user_id.as_bytes().to_vec()
    }

    fn as_prefix(&self) -> [u8; UserId::LENGTH] {
        self.user_id.as_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            user_id: UserId::from_bytes(bytes)?,
        })
    }
}
