use crate::error::{Error, ErrorKind, Result};
use crate::models::Gear;
use rmp_serde as rmps;

#[derive(Clone)]
pub struct GearTree {
    pub(super) usernameid_gear: sled::Tree,
}

impl GearTree {
    pub fn exists(&self, username: &str, gear_id: &str) -> Result<bool> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(gear_id.as_bytes());

        Ok(self.usernameid_gear.contains_key(&key)?)
    }

    pub fn insert(&self, gear: Gear, username: &str) -> Result<()> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(gear.name.as_bytes());

        let serialized = rmps::to_vec(&gear)?;
        self.usernameid_gear.insert(key, serialized)?;

        Ok(())
    }

    pub fn iter(&self, username: &str) -> Result<impl Iterator<Item = Gear>> {
        let mut prefix = username.as_bytes().to_vec();
        prefix.push(0xff);

        Ok(self
            .usernameid_gear
            .scan_prefix(&prefix)
            .values()
            .rev()
            .flatten()
            .flat_map(|x| rmps::from_read_ref(&x)))
    }

    pub fn get(&self, username: &str, gear_id: &str) -> Result<Gear> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(gear_id.as_bytes());

        self.usernameid_gear
            .get(&key)?
            .and_then(|x| rmps::from_read_ref(&x).ok())
            .ok_or(Error::BadRequest(ErrorKind::NotFound, "Gear not found"))
    }
}
