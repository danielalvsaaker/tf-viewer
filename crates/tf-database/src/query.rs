use serde::Deserialize;
use serde::{Serialize, de::DeserializeOwned};
use std::borrow::Cow;
use tf_models::Activity;
use tf_models::gear::Gear;

pub trait Key: Serialize + DeserializeOwned {
    fn as_key(&self) -> Vec<u8>;
    fn as_prefix(&self) -> Vec<u8>;
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GearQuery<'a> {
    pub user_id: Cow<'a, str>,
    pub id: Cow<'a, str>,
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
}

impl<'a> From<&'a Gear> for GearQuery<'a> {
    fn from(gear: &'a Gear) -> Self {
        Self {
            user_id: Cow::Borrowed(&gear.owner),
            id: Cow::Borrowed(&gear.id),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
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
}

impl Key for UserQuery {
    fn as_key(&self) -> Vec<u8> {
        self.user_id.as_bytes().to_vec()
    }

   fn as_prefix(&self) -> Vec<u8> {
        let mut key = self.as_key();
        key.push(0xff);

        key
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

#[derive(Deserialize, Serialize)]
pub struct UserQuery {
    pub user_id: String,
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

impl<'a> From<(&'a UserQuery, &'a UserQuery)> for FollowerQuery<'a> {
    fn from((owner, user): (&'a UserQuery, &'a UserQuery)) -> Self {
        Self {
            owner_id: Cow::Borrowed(&owner.user_id),
            user_id: Cow::Borrowed(&user.user_id),
        }
    }
}
