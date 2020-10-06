use crate::{Error, Result, User};

#[derive(Clone)]
pub struct UserTree {
    pub(super) username_password: sled::Tree,
    pub(super) username_user: sled::Tree,
}

impl UserTree {
    pub fn exists(&self, id: &str) -> Result<bool> {
        Ok(self.username_password.contains_key(id)?)
    }

    pub fn insert(&self, user: User, username: String, hash: String) -> Result<()> {
        let key = username.as_bytes();

        let serialized = bincode::serialize(&user).expect("Failed to serialize user");

        self.username_password.insert(key, hash.as_bytes())?;
        self.username_user.insert(key, serialized)?;

        Ok(())
    }

    pub fn get(&self, id: String) -> Result<User> {
        let key = id.as_bytes();

        let get = self.username_user.get(&key)?;
        Ok(bincode::deserialize::<User>(&get.unwrap()).unwrap())
    }
        
}


