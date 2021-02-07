pub mod activities;
pub mod gear;
pub mod users;

use crate::error::Result;

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
                username_standardgear: db.open_tree("username_standardgear")?,
                username_heartraterest: db.open_tree("username_heartraterest")?,
                username_heartratemax: db.open_tree("username_heartratemax")?,
            },

            activities: activities::ActivityTree {
                usernameid_gearid: db.open_tree("usernameid_gearid")?,
                usernameid_session: db.open_tree("usernameid_session")?,
                usernameid_record: db.open_tree("usernameid_record")?,
                usernameid_lap: db.open_tree("usernameid_lap")?,
                usernameid_notes: db.open_tree("usernameid_notes")?,
            },

            gear: gear::GearTree {
                usernameid_gear: db.open_tree("usernameid_gear")?,
            },

            _db: db,
        })
    }
}
