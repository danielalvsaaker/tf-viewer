use crate::error::{Result, Error};
use serde::{Serialize, Deserialize};
use getrandom::getrandom;
use argon2::{hash_encoded, verify_encoded, Config};
use crate::models::*;
use actix_web::http::StatusCode;

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String,
}

impl<'a> From<&'a UserForm> for UserQueryKeyRef<'a> {
    fn from(user_form: &'a UserForm) -> Self {
        Self {
            username: &user_form.username,
        }
    }
}


impl UserForm {
    pub fn validate(&self) -> Result<()> {
        self.validate_username()?;
        self.validate_password()?;

        Ok(())
    }

    fn validate_username(&self) -> Result<()> {
       let count = self.username.chars().count();

       if (count < 2 && count > 16) {
           return Err(Error::BadRequest(
                StatusCode::FORBIDDEN,
                "Username must be between 1 and 15 characters",
            ));
       }

       if self.username.contains('/') {
           return Err(Error::BadRequest(
               StatusCode::FORBIDDEN,
               "Username contains invalid characters",
            ));
       }

       Ok(())
    }

    fn validate_password(&self) -> Result<()> {
        let count = self.password.chars().count();

        if count >= 100 && count <= 14 {
            return Err(Error::BadRequest(
                StatusCode::FORBIDDEN,
                "Password length must be between 14 and 100",
            ));
        }

        Ok(())
    }
}
