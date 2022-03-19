use crate::{error::Result, Key};
use serde::Deserialize;
use tf_models::{ActivityId, UserId};

#[derive(Clone, Copy, Deserialize)]
pub struct ActivityQuery {
    pub user_id: UserId,
    pub id: ActivityId,
}

impl Key for ActivityQuery {
    fn as_key(&self) -> Vec<u8> {
        [
            self.user_id.as_bytes().as_slice(),
            self.id.as_bytes().as_slice(),
        ]
        .concat()
    }

    fn as_prefix(&self) -> [u8; UserId::LENGTH] {
        self.user_id.as_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let (prefix, suffix) = bytes.split_at(UserId::LENGTH);

        Ok(Self {
            user_id: UserId::from_bytes(prefix)?,
            id: ActivityId::from_bytes(suffix)?,
        })
    }
}
