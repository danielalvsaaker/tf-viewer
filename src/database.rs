use crate::{Error, Result};

pub mod users;
pub mod activities;
pub mod gear;

#[derive(Clone)]
pub struct Database {
    pub users: users::UserTree,
    pub activities: activities::ActivityTree,
    pub gear: gear::GearTree,
    pub _db: sled::Db,
}

impl Database {
    pub fn load_or_create() -> Result<Self> {
        let db = sled::open("db")?;

        Ok(Self {
            users: users::UserTree {
                username_password: db.open_tree("username_password")?,
                username_user: db.open_tree("username_user")?,
            },

            activities: activities::ActivityTree {
                usernameid_activity: db.open_tree("usernameid_activity")?,
            },

            gear: gear::GearTree {
                usernameid_gear: db.open_tree("usernameid_gear_type")?,
            },

            _db: db,
        })
    }
}


