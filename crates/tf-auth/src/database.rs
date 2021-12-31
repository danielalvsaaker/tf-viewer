use crate::{error::Result, routes::UserForm};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Clone)]
pub struct Database {
    pub(super) username_password: sled::Tree,
    pub(super) _db: sled::Db,
}

impl Database {
    pub fn load_or_create() -> Result<Self> {
        let db = sled::Config::new()
            .path("auth-db")
            .use_compression(true)
            .open()?;

        Ok(Self {
            username_password: db.open_tree("username_password")?,
            _db: db,
        })
    }

    pub fn insert(&self, user: &UserForm) -> Result<()> {
        let salt = SaltString::generate(&mut OsRng);

        let hash = Argon2::default()
            .hash_password(user.password.as_bytes(), &salt)?
            .to_string();

        self.username_password.insert(&user.username, &hash)?;

        Ok(())
    }

    pub fn verify_hash(&self, user: &UserForm) -> Result<bool> {
        Ok(self
            .username_password
            .get(&user.username)?
            .as_deref()
            .map(std::str::from_utf8)
            .transpose()
            .unwrap_or_default()
            .map(PasswordHash::new)
            .transpose()?
            .map(|x| {
                Argon2::default()
                    .verify_password(user.password.as_bytes(), &x)
                    .is_ok()
            })
            .unwrap_or_default())
    }
}
