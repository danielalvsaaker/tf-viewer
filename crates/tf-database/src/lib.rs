pub mod error;
pub mod primitives;
pub mod query;
pub mod resource;
pub mod root;

use error::Result;
use query::{ActivityQuery, ClientQuery, GearQuery, UserQuery};
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

    pub fn compact(&self) -> Result<()> {
        Ok(self._db.compact()?)
    }

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

use tf_models::{
    activity::*,
    gear::Gear,
    user::{Password, User},
};

pub trait Traverse<'a, T: Resource> {
    type Collection;
}

impl<'a> Traverse<'a, Gear> for Session {
    type Collection = primitives::Relation<'a, ActivityQuery, Session, GearQuery, Gear>;
}

impl<'a> Traverse<'a, User> for Session {
    type Collection = primitives::Relation<'a, ActivityQuery, Session, UserQuery, User>;
}

impl<'a> Traverse<'a, Session> for User {
    type Collection = primitives::Relation<'a, ActivityQuery, Session, UserQuery, User>;
}

impl<'a> Traverse<'a, Record> for User {
    type Collection = primitives::Relation<'a, ActivityQuery, Record, UserQuery, User>;
}

impl<'a> Traverse<'a, Vec<Lap>> for User {
    type Collection = primitives::Relation<'a, ActivityQuery, Vec<Lap>, UserQuery, User>;
}

impl<'a> Traverse<'a, Gear> for User {
    type Collection = primitives::Relation<'a, GearQuery, Gear, UserQuery, User>;
}

impl<'a> Traverse<'a, User> for Gear {
    type Collection = primitives::Relation<'a, GearQuery, Gear, UserQuery, User>;
}
