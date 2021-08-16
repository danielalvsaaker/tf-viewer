use crate::error::{Error, Result};
use std::convert::TryInto;
use crate::models::*;
use argon2::{hash_encoded, verify_encoded, Config};
use actix_web::http::StatusCode;

#[derive(Clone)]
pub struct UserTree {
    pub(super) username_password: sled::Tree,
    pub(super) username_standardgear: sled::Tree,
    pub(super) username_heartraterest: sled::Tree,
    pub(super) username_heartratemax: sled::Tree,
}

impl UserTree {
    pub fn exists(&self, user: &UserQuery) -> Result<()> {
        match self.username_password.contains_key(user.to_key())? {
            true => Ok(()),
            false => Err(Error::BadRequest(StatusCode::NOT_FOUND, "User not found")),
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

    pub fn set_standard_gear<Q: Query>(&self, query: &Q) -> Result<()> {
        self.username_standardgear.insert(query.username(), query.id())?;

        Ok(())
    }

    pub fn get_standard_gear<U: UserQuery>(&self, user: &U) -> Result<Option<String>> {
        let get = self.username_standardgear.get(user.to_key())?;

        match get {
            Some(x) => Ok(String::from_utf8(x.to_vec()).ok()),
            None => Ok(None),
        }
    }

    pub fn set_heartrate<U: UserQuery>(
        &self,
        user: &U,
        (heartraterest, heartratemax): (u8, u8),
    ) -> Result<()> {
        self.username_heartraterest
            .insert(user.to_key(), &heartraterest.to_ne_bytes())?;
        self.username_heartratemax
            .insert(user.to_key(), &heartratemax.to_ne_bytes())?;

        Ok(())
    }

    pub fn get_heartrate<U: UserQuery>(&self, user: &U) -> Result<Option<(u8, u8)>> {
        let heartraterest = self.username_heartraterest.get(user.to_key())?;
        let heartratemax = self.username_heartratemax.get(user.to_key())?;

        if let (Some(x), Some(y)) = (heartraterest, heartratemax) {
            Ok(Some((
                u8::from_ne_bytes(
                    x.as_ref()
                        .try_into()
                        .map_err(|_| Error::BadServerResponse("Failed to get heart rate"))?,
                ),
                u8::from_ne_bytes(
                    y.as_ref()
                        .try_into()
                        .map_err(|_| Error::BadServerResponse("Failed to get heart rate"))?,
                ),
            )))
        } else {
            Ok(None)
        }
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

    pub fn iter_id(&self) -> Result<impl Iterator<Item = String>> {
        Ok(self
            .username_password
            .iter()
            .keys()
            .flatten()
            .flat_map(|x| String::from_utf8(x.to_vec())))
    }
}
