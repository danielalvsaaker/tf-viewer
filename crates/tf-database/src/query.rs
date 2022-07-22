use crate::{error::Result, primitives::Key};
pub use tf_models::query::{ActivityQuery, ClientQuery, GearQuery, UserQuery};
use tf_models::{ActivityId, ClientId, GearId, UserId};

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

impl Key for ClientQuery {
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
            id: ClientId::from_bytes(suffix)?,
        })
    }
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
