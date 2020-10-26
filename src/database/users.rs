use crate::User;
use argonautica::{Hasher, Verifier};
use dotenv::var;
use anyhow::{anyhow, Result};

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
        let serialized = bincode::serialize(&user)?;

        let mut hasher = Hasher::default();

        let hash = hasher
            .with_password(password)
            .with_secret_key(var("PASSWORD")?)
            .hash()
            .unwrap();


        self.username_password.insert(username, hash.as_bytes())?;
        self.username_user.insert(username, serialized)?;

        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<User> {
        let get = self.username_user.get(id)?;

        match get {
            Some(x) => Ok(bincode::deserialize::<User>(&x)?),
            None => Err(anyhow!("Failed to deserialize user")),
        }
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
        let hash = String::from_utf8(self.username_password.get(&id)?.unwrap().to_vec())?;
        let mut verifier = Verifier::default();
        
        Ok(verifier
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(var("PASSWORD")?)
            .verify().unwrap())

    }

    pub fn iter_user(&self) -> Result<impl Iterator<Item = User>> {

        Ok(self.username_user.iter()
           .values()
           .map(|x| bincode::deserialize::<User>(&x.unwrap()).unwrap())
           )
    }

    pub fn iter_id(&self) -> Result<impl Iterator<Item = String>> {

        Ok(self.username_user.iter()
           .keys()
           .map(|x| String::from_utf8(x.unwrap().to_vec()).unwrap())
           )
    }
}


