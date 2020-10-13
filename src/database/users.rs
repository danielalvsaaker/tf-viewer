use crate::{Error, Result, User};
use argonautica::{Hasher, Verifier};
use dotenv::var;
use futures::future::*;

#[derive(Clone)]
pub struct UserTree {
    pub(super) username_password: sled::Tree,
    pub(super) username_user: sled::Tree,
}

impl UserTree {
    pub fn exists(&self, id: &str) -> Result<bool> {
        Ok(self.username_password.contains_key(id)?)
    }

    pub fn insert(&self, user: User, username: &str, password: &str) -> Result<()> {
        let serialized = bincode::serialize(&user).expect("Failed to serialize user");

        let mut hasher = Hasher::default();

        let hash = hasher
            .with_password(password)
            .with_secret_key(var("PASSWORD").unwrap())
            .hash()
            .unwrap();


        self.username_password.insert(username, hash.as_bytes())?;
        self.username_user.insert(username, serialized)?;

        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<User> {
        let get = self.username_user.get(id)?;

        Ok(bincode::deserialize::<User>(&get.unwrap()).unwrap())
    }

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

    pub fn verify_hash(&self, id: &str, password: &str) -> Result<bool> {
        let hash = String::from_utf8(self.username_password.get(&id)?.unwrap().to_vec()).unwrap();
        let mut verifier = Verifier::default();
        
        Ok(verifier
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(var("PASSWORD").unwrap())
            .verify().unwrap())

    }
}


