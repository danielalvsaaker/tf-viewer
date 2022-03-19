pub mod error;
pub mod primitives;
pub mod query;
pub mod resource;
pub mod root;

use error::Result;
use query::{ActivityQuery, GearQuery, UserQuery};
use resource::Resource;
use root::Root;

#[derive(Clone)]
pub struct Database {
    _db: primitives::Database,
}

impl Database {
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        Ok(Self {
            _db: primitives::Database::open(path)?,
        })
    }

    /*
    pub fn create_temporary() -> Result<Self> {
        Ok(Self {
            _db: primitives::Database::create_temporary()?,
        })
    }
    */

    pub fn root<R>(&self) -> Result<Root<'_, R, primitives::Tree<R::Key, R>>>
    where
        R: Resource,
    {
        Ok(Root {
            _resource: Default::default(),
            _db: &self._db,
            collection: self._db.open_resource()?,
        })
    }
}

use tf_models::{activity::*, gear::Gear, user::User};

pub trait Traverse<T: Resource> {
    type Collection;
}

impl Traverse<Gear> for Session {
    type Collection = primitives::Relation<ActivityQuery, Session, GearQuery, Gear>;
}

impl Traverse<User> for Session {
    type Collection = primitives::Relation<ActivityQuery, Session, UserQuery, User>;
}

impl Traverse<Session> for User {
    type Collection = primitives::Relation<ActivityQuery, Session, UserQuery, User>;
}

impl Traverse<Record> for User {
    type Collection = primitives::Relation<ActivityQuery, Record, UserQuery, User>;
}

impl Traverse<Vec<Lap>> for User {
    type Collection = primitives::Relation<ActivityQuery, Vec<Lap>, UserQuery, User>;
}

impl Traverse<Gear> for User {
    type Collection = primitives::Relation<GearQuery, Gear, UserQuery, User>;
}

impl Traverse<User> for Gear {
    type Collection = primitives::Relation<GearQuery, Gear, UserQuery, User>;
}
