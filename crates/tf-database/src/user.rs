use crate::error::Result;
use crate::query::{FollowerQuery, GearQuery, UserQuery, Key};
use rmp_serde as rmps;
use tf_models::user::User;

#[derive(serde::Serialize, serde::Deserialize)]
enum Visibility {
    Public,
    Private,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Private
    }
}

#[derive(Clone)]
pub struct UserTree {
    pub(super) username_user: sled::Tree,
    pub(super) username_standardgearid: sled::Tree,
    pub(super) username_visibility: sled::Tree,
    pub(super) usernamefollower_follower: sled::Tree,
    pub(super) usernamefollower_request: sled::Tree,
}

impl UserTree {
    /*
    pub fn has_access(&self, owner: &UserQuery, user: &UserQuery) -> Result<bool> {
        let owner_key = owner.as_key();

        let vis: Visibility = self
            .username_visibility
            .get(&owner_key)?
            .as_deref()
            .map(rmps::from_read_ref)
            .transpose()?
            .unwrap_or_default();

        if let Visibility::Public = vis {
            Ok(true)
        } else {
            let key = FollowerQuery::from((owner, user)).as_key();
            if self.usernamefollower_follower.contains_key(&key)? {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    pub fn follow(&self, owner: &UserQuery, user: &UserQuery) -> Result<()> {
        let owner_key = owner.as_key();

        let vis: Visibility = self
            .username_visibility
            .get(&owner_key)?
            .as_deref()
            .map(rmps::from_read_ref)
            .transpose()?
            .unwrap_or_default();

        let key = FollowerQuery::from((owner, user)).as_key();
        if let Visibility::Public = vis {
            self.usernamefollower_follower
                .insert(&key, rmps::to_vec(&user.user_id)?)?;
        } else {
            self.usernamefollower_request
                .insert(&key, rmps::to_vec(&user.user_id)?)?;
        }

        Ok(())
    }

    pub fn unfollow(&self, owner: &UserQuery, user: &UserQuery) -> Result<()> {
        let key = FollowerQuery::from((owner, user)).as_key();

        self.usernamefollower_follower.remove(&key)?;

        Ok(())
    }

    pub fn requests(&self, user: &UserQuery) -> Result<Option<impl Iterator<Item = String>>> {
        let key = user.as_key();

        let vis: Visibility = self
            .username_visibility
            .get(key)?
            .as_deref()
            .map(rmps::from_read_ref)
            .transpose()?
            .unwrap_or_default();

        if let Visibility::Public = vis {
            Ok(None)
        } else {
            Ok(Some(
                self.usernamefollower_request
                    .scan_prefix(user.to_prefix())
                    .keys()
                    .flatten()
                    .flat_map(|x| rmps::from_read_ref(&x)),
            ))
        }
    }
*/

    /*

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
        Ok(self
            .username_user
            .insert(&query.as_key(), rmps::to_vec(user)?)?
            .as_deref()
            .map(|_| ()))
    }

    pub fn remove_user(&self, query: &UserQuery) -> Result<()> {
        self.username_user.remove(&query.as_key())?;

        Ok(())
    }

    pub fn get_user(&self, query: &UserQuery) -> Result<Option<User>> {
        Ok(self
            .username_user
            .get(&query.as_key())?
            .as_deref()
            .map(|x| rmps::from_read_ref(&x))
            .transpose()?)
    }

    /*
    pub fn set_standard_gear(&self, query: &GearQuery) -> Result<Option<()>> {
        let user_query = UserQuery::from(query);

        Ok(self
            .username_standardgearid
            .insert(&user_query.as_key(), rmps::to_vec(&query.id)?)?
            .as_deref()
            .map(|_| ()))
    }
    */

    pub fn get_standard_gear(&self, user: &UserQuery) -> Result<Option<String>> {
        Ok(self
            .username_standardgearid
            .get(&user.as_key())?
            .as_deref()
            .map(|x| rmps::from_read_ref(&x))
            .transpose()?)
    }
}
