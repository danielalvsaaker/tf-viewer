/*pub mod activities;
pub mod gear;
pub mod users;

use anyhow::Result;

*/
#[derive(Clone)]
pub struct Database {
    pub user: user::UserTree,
    pub activity: activity::ActivityTree,
    pub gear: gear::GearTree,
    pub _db: sled::Db,
}

impl Database {
    pub fn load_or_create() -> Result<Self> {
        let db = sled::Config::new()
            .path("db")
            .use_compression(true)
            .open()?;

        Ok(Self {
            user: user::UserTree {
                username_user: db.open_tree("username_user")?,
                username_standardgearid: db.open_tree("username_standardgearid")?,
            },

            activity: activity::ActivityTree {
                usernameid_session: db.open_tree("usernameid_session")?,
                usernameid_record: db.open_tree("usernameid_record")?,
                usernameid_lap: db.open_tree("usernameid_lap")?,
                usernameid_gearid: db.open_tree("usernameid_gearid")?,
            },

            gear: gear::GearTree {
                usernameid_gear: db.open_tree("usernameid_gear")?,
            },

            _db: db,
        })
    }

    pub fn generate_id(&self) -> Result<u64> {
        Ok(self._db.generate_id()?)
    }
}

pub mod error;
pub mod query;
pub use error::Result;

pub mod activity;
pub mod gear;
pub mod user;
