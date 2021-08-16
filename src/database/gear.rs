use crate::error::{Error, Result};
use crate::models::{Gear, UserQuery, Query, QueryKeyRef};
use actix_web::http::StatusCode;
use rmp_serde as rmps;

#[derive(Clone)]
pub struct GearTree {
    pub(super) usernameid_gear: sled::Tree,
}

impl GearTree {
    pub fn exists<Q: Query>(&self, query: &Q) -> Result<bool> {
        Ok(self.usernameid_gear.contains_key(&query.to_key())?)
    }

    pub fn insert(&self, gear: Gear) -> Result<()> {
        let query: QueryKeyRef = (&gear).into();

        let serialized = rmps::to_vec(&gear)?;
        self.usernameid_gear.insert(query.to_key(), serialized)?;

        Ok(())
    }

    pub fn iter(&self, user: &UserQuery) -> Result<impl Iterator<Item = Gear>> {
        Ok(self
            .usernameid_gear
            .scan_prefix(&user.to_prefix())
            .values()
            .rev()
            .flatten()
            .flat_map(|x| rmps::from_read_ref(&x)))
    }

    pub fn get<Q: Query>(&self, query: &Q) -> Result<Gear> {
        self.usernameid_gear
            .get(&query.to_key())?
            .and_then(|x| rmps::from_read_ref(&x).ok())
            .ok_or(Error::BadRequest(StatusCode::NOT_FOUND, "Gear not found"))
    }
}
