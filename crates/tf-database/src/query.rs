use serde::Deserialize;
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;
use tf_models::gear::Gear;
use tf_models::user::User;
use tf_models::Activity;

pub trait Key: Serialize + DeserializeOwned {
    fn as_key(&self) -> Vec<u8>;
    fn as_prefix(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GearQuery<'a> {
    pub user_id: Cow<'a, str>,
    pub id: Cow<'a, str>,
}

impl<'a> From<&'a ActivityQuery<'a>> for UserQuery<'a> {
    fn from(query: &'a ActivityQuery) -> Self {
        Self {
            user_id: Cow::Borrowed(&query.user_id),
        }
    }
}

impl Key for GearQuery<'_> {
    fn as_key(&self) -> Vec<u8> {
        let mut key = self.as_prefix();
        key.extend_from_slice(self.id.as_bytes());

        key
    }

    fn as_prefix(&self) -> Vec<u8> {
        let mut key = self.user_id.as_bytes().to_vec();
        key.push(0xff);

        key
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut fields = bytes
            .rsplit(|&b| b == 0xff)
            .map(Into::into)
            .flat_map(String::from_utf8);

        Some(Self {
            user_id: fields.next()?.into(),
            id: fields.next()?.into(),
        })
    }
}

impl<'a> From<&'a Gear> for GearQuery<'a> {
    fn from(gear: &'a Gear) -> Self {
        Self {
            user_id: Cow::Borrowed(&gear.owner),
            id: Cow::Borrowed(&gear.id),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActivityQuery<'a> {
    pub user_id: Cow<'a, str>,
    pub id: Cow<'a, str>,
}

impl Key for ActivityQuery<'_> {
    fn as_key(&self) -> Vec<u8> {
        let mut key = self.as_prefix();
        key.extend_from_slice(self.id.as_bytes());

        key
    }

    fn as_prefix(&self) -> Vec<u8> {
        let mut key = self.user_id.as_bytes().to_vec();
        key.push(0xff);

        key
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut fields = bytes
            .rsplit(|&b| b == 0xff)
            .map(Into::into)
            .flat_map(String::from_utf8);

        Some(Self {
            user_id: fields.next()?.into(),
            id: fields.next()?.into(),
        })
    }
}

impl Key for UserQuery<'_> {
    fn as_key(&self) -> Vec<u8> {
        self.user_id.as_bytes().to_vec()
    }

    fn as_prefix(&self) -> Vec<u8> {
        let mut key = self.as_key();
        key.push(0xff);

        key
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(Self {
            user_id: String::from_utf8(bytes.into()).ok()?.into(),
        })
    }
}

impl<'a> From<&'a Activity> for ActivityQuery<'a> {
    fn from(activity: &'a Activity) -> Self {
        Self {
            user_id: Cow::Borrowed(&activity.owner),
            id: Cow::Borrowed(&activity.id),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserQuery<'a> {
    pub user_id: Cow<'a, str>,
}

pub struct FollowerQuery<'a> {
    pub owner_id: Cow<'a, str>,
    pub user_id: Cow<'a, str>,
}

impl<'a> FollowerQuery<'a> {
    pub fn to_key(&self) -> Vec<u8> {
        let mut key = self.owner_id.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(self.user_id.as_bytes());

        key
    }
}
