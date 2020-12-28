use anyhow::Result;

pub mod activities;
pub mod gear;
pub mod users;

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
                username_standard_gear: db.open_tree("username_standard_gear")?,
                username_heartrate_rest: db.open_tree("username_heartrate_rest")?,
                username_heartrate_max: db.open_tree("username_heartrate_max")?,
            },

            activities: activities::ActivityTree {
                usernameid_username: db.open_tree("usernameid_username")?,
                usernameid_id: db.open_tree("usernameid_id")?,
                usernameid_gear: db.open_tree("usernameid_gear")?,
                usernamegearid_id: db.open_tree("usernamegearid_id")?,
                usernameid_session: db.open_tree("usernameid_session")?,
                usernameid_record: db.open_tree("usernameid_record")?,
                usernameid_lap: db.open_tree("usernameid_lap")?,
            },

            gear: gear::GearTree {
                usernameid_gear: db.open_tree("usernameid_gear_type")?,
            },

            _db: db,
        })
    }
}
