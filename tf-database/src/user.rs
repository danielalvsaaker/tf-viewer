use crate::error::Result;
use crate::query::{GearQuery, UserQuery};
use tf_models::backend::User;
use rmp_serde as rmps;

#[derive(Clone)]
pub struct UserTree {
    pub(super) username_user: sled::Tree,
    pub(super) username_standardgearid: sled::Tree,
}

impl UserTree {
    /*
    pub fn has_access(&self, user: &UserQuery, owner: &UserQuery) -> Result<bool> {
        if self.username_private.contains_key(owner.to_key())? {
            if self.usernamefollows_follows.contains_key(
        }
    }

    pub fn insert(&self, user: UserForm) -> Result<()> {
        let mut salt = [0u8; 32];
        getrandom::getrandom(&mut salt).unwrap();

        let hash = hash_encoded(user.password.as_bytes(), &salt, &Config::default())
            .map_err(|_| Error::BadServerResponse("Password hashing failed"))?;

        self.username_password.insert(&user.username, hash.as_bytes())?;

        Ok(())
    }

    pub fn verify_hash(&self, user: &UserForm) -> Result<bool> {
        let hash = String::from_utf8(
            self.username_password
                .get(&user.username)?
                .ok_or(Error::BadRequest(
                    StatusCode::NOT_FOUND,
                    "Password not found in database",
                ))?
                .to_vec(),
        )
        .map_err(|_| Error::BadServerResponse("Password in database is invalid"))?;

        match verify_encoded(&hash, user.password.as_bytes()) {
            Ok(true) => Ok(true),
            _ => Err(Error::BadRequest(StatusCode::UNAUTHORIZED, "Incorrect password")),
        }
    }

    */
    pub fn insert_user(&self, query: &UserQuery, user: &User) -> Result<Option<()>> {
        Ok(self.username_user
            .insert(&query.to_key(), rmps::to_vec(user)?)?
            .as_deref()
            .map(|_| ()))
    }

    pub fn remove_user(&self, query: &UserQuery) -> Result<()> {
        self.username_user
            .remove(&query.to_key())?;

        Ok(())
    }

    pub fn get_user(&self, query: &UserQuery) -> Result<Option<User>> {
        Ok(self.username_user
            .get(&query.to_key())?
            .as_deref()
            .map(|x| rmps::from_read_ref(&x))
            .transpose()?)
    }

    pub fn set_standard_gear(&self, query: &GearQuery) -> Result<Option<()>> {
        let user_query = UserQuery::from(query);

        Ok(self
            .username_standardgearid
            .insert(&user_query.to_key(), rmps::to_vec(&query.id)?)?
            .as_deref()
            .map(|_| ()))
    }

    pub fn get_standard_gear(&self, user: &UserQuery) -> Result<Option<String>> {
        Ok(self
            .username_standardgearid
            .get(&user.to_key())?
            .as_deref()
            .map(|x| rmps::from_read_ref(&x))
            .transpose()?)
    }
}
