pub mod error;
pub mod primitives;
pub mod query;
pub mod resource;
pub mod root;

use self::{error::Result, resource::Resource, root::Root};

#[derive(Clone)]
pub struct Database {
    db: primitives::Database,
}

impl Database {
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        Ok(Self {
            db: primitives::Database::open(path)?,
        })
    }

    pub fn compact(&self) -> Result<()> {
        self.db.compact()
    }

    pub fn root<R>(&self) -> Result<Root<'_, R, primitives::Tree<R::Key, R>>>
    where
        R: Resource,
    {
        Ok(Root {
            _resource: Default::default(),
            db: &self.db,
            collection: self.db.open_resource()?,
        })
    }
}

pub trait Traverse<T: Resource> {
    type Collection;
}
