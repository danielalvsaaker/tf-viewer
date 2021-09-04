use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize)]
pub struct GearQuery<'a> {
    pub user_id: Cow<'a, str>,
    pub id: Cow<'a, str>,
}

impl<'a> GearQuery<'a> {
    pub fn to_key(&self) -> Vec<u8> {
        let mut key = self.user_id.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(self.id.as_bytes());

        key
    }
}

#[derive(Deserialize)]
pub struct ActivityQuery<'a> {
    pub user_id: Cow<'a, str>,
    pub id: Cow<'a, str>,
}

impl<'a> ActivityQuery<'a> {
    pub fn to_key(&self) -> Vec<u8> {
        let mut key = self.user_id.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(self.id.as_bytes());

        key
    }
}

#[derive(Deserialize)]
pub struct UserQuery<'a> {
    pub user_id: Cow<'a, str>,
}

impl<'a> From<&'a str> for UserQuery<'a> {
    fn from(user_id: &'a str) -> Self {
        Self {
            user_id: Cow::from(user_id),
        }
    }
}

impl<'a> From<&'a GearQuery<'a>> for UserQuery<'a> {
    fn from(q: &'a GearQuery) -> Self {
        Self {
            user_id: Cow::Borrowed(&q.user_id),
        }
    }
}

impl<'a> From<&'a ActivityQuery<'a>> for UserQuery<'a> {
    fn from(q: &'a ActivityQuery<'a>) -> Self {
        Self {
            user_id: Cow::Borrowed(&q.user_id),
        }
    }
}

impl<'a> From<(&'a UserQuery<'a>, &'a str)> for GearQuery<'a> {
    fn from((query, id): (&'a UserQuery<'a>, &'a str)) -> Self {
        Self {
            user_id: Cow::Borrowed(&query.user_id),
            id: Cow::from(id),
        }
    }
}

impl<'a> From<(&'a UserQuery<'a>, &'a str)> for ActivityQuery<'a> {
    fn from((query, id): (&'a UserQuery<'a>, &'a str)) -> Self {
        Self {
            user_id: Cow::Borrowed(&query.user_id),
            id: Cow::from(id),
        }
    }
}

impl<'a> UserQuery<'a> {
    pub fn to_key(&'_ self) -> &'_ [u8] {
        self.user_id.as_bytes()
    }

    pub fn to_prefix(&self) -> Vec<u8> {
        let mut key = self.to_key().to_vec();
        key.push(0xff);

        key
    }
}
