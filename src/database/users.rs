use crate::User;
use anyhow::{anyhow, Result};
use argon2::{hash_encoded, verify_encoded, Config};
use getrandom::getrandom;

#[derive(Clone)]
pub struct UserTree {
    pub(super) username_password: sled::Tree,
    pub(super) username_user: sled::Tree,
    pub(super) username_standard_gear: sled::Tree,
}

impl UserTree {
    pub fn exists(&self, id: &str) -> Result<bool> {
        Ok(self.username_password.contains_key(id)?)
    }

    pub fn insert(&self, user: User, username: &str, password: &str) -> Result<()> {
        let serialized = bincode::serialize(&user)?;

        let mut salt = [0u8; 32];
        getrandom(&mut salt).unwrap();

        let hash = hash_encoded(password.as_bytes(), &salt, &Config::default())?;

        self.username_password.insert(username, hash.as_bytes())?;
        self.username_user.insert(username, serialized)?;

        Ok(())
    }

    pub fn set_standard_gear(&self, username: &str, gear: &str) -> Result<()> {
        self.username_standard_gear.insert(username, gear)?;

        Ok(())
    }

    pub fn get_standard_gear(&self, username: &str) -> Result<Option<String>> {
        let get = self.username_standard_gear.get(username)?;

        match get {
            Some(x) => Ok(Some(String::from_utf8(x.to_vec())?)),
            None => Ok(None),
        }
    }

    pub fn get(&self, id: &str) -> Result<User> {
        let get = self.username_user.get(id)?;

        match get {
            Some(x) => Ok(bincode::deserialize::<User>(&x)?),
            None => Err(anyhow!("User not found")),
        }
    }

    /*
    pub fn zones(&self, id: &str) -> Result<Vec<u8>> {
        let user = self.get(id)?;

        let mut zones: Vec<u8> = vec![];

        zones.push(user.heartrate_rest);
        zones.push(user.heartrate_max * 55 / 10);
        zones.push(user.heartrate_max * 72 / 10);
        zones.push(user.heartrate_max * 82 / 10);
        zones.push(user.heartrate_max * 87 / 10);
        zones.push(user.heartrate_max * 92 / 10);

        Ok(zones)
    }
    */

    pub fn verify_hash(&self, id: &str, password: &str) -> Result<bool> {
        let hash = String::from_utf8(self.username_password.get(&id)?.unwrap().to_vec())?;

        Ok(verify_encoded(&hash, password.as_bytes())?)
    }

    pub fn iter_user(&self) -> Result<impl Iterator<Item = User>> {
        Ok(self
            .username_user
            .iter()
            .values()
            .flatten()
            .flat_map(|x| bincode::deserialize::<User>(&x)))
    }

    pub fn iter_id(&self) -> Result<impl Iterator<Item = String>> {
        Ok(self
            .username_user
            .iter()
            .keys()
            .flatten()
            .flat_map(|x| String::from_utf8(x.to_vec())))
    }
}
