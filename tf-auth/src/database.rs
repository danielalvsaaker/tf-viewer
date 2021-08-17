use crate::error::{Error, Result};
use std::convert::TryInto;
use crate::models::*;
use argon2::{hash_encoded, verify_encoded, Config};
use actix_web::http::StatusCode;

#[derive(Clone)]
pub struct Database {
    pub(super) username_password: sled::Tree,
}

impl Database {
    pub fn exists(&self, user: &UserQuery) -> Result<()> {
        Ok(self.username_password.contains_key(user.to_key())?)
    }

    pub fn insert(&self, user: UserForm) -> Result<()> {
        let mut salt = [0u8; 32];
        getrandom::getrandom(&mut salt).unwrap();

        let hash = hash_encoded(user.password.as_bytes(), &salt, &Config::default())
            .map_err(|_| OwnerConsent::Error(WebError::InternalError(
                Some("Failed to hash password")
            )))?;

        self.username_password.insert(&user.username, hash.as_bytes())?;

        Ok(())
    }

    pub fn verify_hash(&self, user: &UserForm) -> Result<OwnerConsent> {
        let hash = String::from_utf8(
            self.username_password
                .get(&user.username)?
                .ok_or(OwnerConsent::Denied)?
                .to_vec(),
        )
        .map_err(|_| OwnerConsent::Denied)?;

        match verify_encoded(&hash, user.password.as_bytes()) {
            Ok(true) => Ok(OwnerConsent::Authorized(user.username)),
            _ => Err(OwnerConsent::Denied),
        }
    }
}
