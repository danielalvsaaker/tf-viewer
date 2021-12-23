use crate::error::Result;
use crate::query::{GearQuery, UserQuery};
use rmp_serde as rmps;
use tf_models::backend::Gear;

#[derive(Clone)]
pub struct GearTree {
    pub(super) usernameid_gear: sled::Tree,
}

impl GearTree {
    pub fn contains_gear(&self, query: &GearQuery) -> Result<bool> {
        Ok(self.usernameid_gear.contains_key(&query.to_key())?)
    }

    pub fn insert_gear(&self, query: &GearQuery, gear: Gear) -> Result<()> {
        self.usernameid_gear
            .insert(&query.to_key(), rmps::to_vec(&gear)?)?;

        Ok(())
    }

    pub fn remove_gear(&self, query: &GearQuery) -> Result<()> {
        self.usernameid_gear.remove(&query.to_key())?;

        Ok(())
    }

    pub fn get_gear(&self, query: &GearQuery) -> Result<Option<Gear>> {
        Ok(self
            .usernameid_gear
            .get(&query.to_key())?
            .as_deref()
            .map(|x| rmps::from_read_ref(&x))
            .transpose()?)
    }

    pub fn iter_gear(&self, query: &UserQuery) -> Result<impl Iterator<Item = Gear>> {
        Ok(self
            .usernameid_gear
            .scan_prefix(&query.to_prefix())
            .values()
            .rev()
            .flatten()
            .flat_map(|x| rmps::from_read_ref(&x)))
    }

    /*
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
    */
}
