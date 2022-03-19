use crate::{error::Result, primitives::Key};
use serde::Deserialize;
use tf_models::{ActivityId, GearId, UserId};

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

#[derive(Clone, Copy, Deserialize)]
pub struct GearQuery {
    pub user_id: UserId,
    pub id: GearId,
}

impl Key for GearQuery {
    fn as_key(&self) -> Vec<u8> {
        [
            self.user_id.as_bytes().as_slice(),
            self.id.as_bytes().as_slice(),
        ]
        .concat()
    }

    fn as_prefix(&self) -> [u8; 21] {
        self.user_id.as_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let (prefix, suffix) = bytes.split_at(UserId::LENGTH);

        Ok(Self {
            user_id: UserId::from_bytes(prefix)?,
            id: GearId::from_bytes(suffix)?,
        })
    }
}

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
