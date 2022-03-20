mod relation;
mod tree;

mod key;
mod value;

pub use key::Key;
pub use relation::Relation;
pub use tree::Tree;
pub use value::Value;

use crate::{error::Result, Resource};
use nebari::tree::Root;

pub type Inner = nebari::Roots<nebari::io::fs::StdFile>;

#[derive(Clone, Copy, Debug)]
struct LZ4Vault;

impl nebari::Vault for LZ4Vault {
    type Error = lz4_flex::block::DecompressError;

    fn encrypt(&self, payload: &[u8]) -> Result<Vec<u8>, Self::Error> {
        Ok(lz4_flex::compress_prepend_size(payload))
    }

    fn decrypt(&self, payload: &[u8]) -> Result<Vec<u8>, Self::Error> {
        Ok(lz4_flex::decompress_size_prepended(payload)?)
    }
}

#[derive(Clone)]
pub struct Database {
    inner: Inner,
}

pub trait OpenCollection<C> {
    fn open_collection(&self) -> Result<C>;
}

impl<R> OpenCollection<Tree<R::Key, R>> for Database
where
    R: Resource,
{
    fn open_collection(&self) -> Result<Tree<R::Key, R>> {
        self.open_resource()
    }
}

impl<R, S> OpenCollection<Relation<R::Key, R, S::Key, S>> for Database
where
    R: Resource,
    S: Resource,
{
    fn open_collection(&self) -> Result<Relation<R::Key, R, S::Key, S>> {
        self.open_relation()
    }
}

impl Database {
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let this = Self {
            inner: nebari::Config::default_for(path)
                .vault(LZ4Vault)
                .open()?,
        };
        this.compact()?;
        Ok(this)
    }

    /*
    pub fn create_temporary() -> Result<Self> {
        Ok(Self {
            inner: sled::Config::new().temporary(true).open()?,
        })
    }
    */

    pub fn compact(&self) -> Result<()> {
        for name in self.inner.tree_names()? {
            self.inner.tree(nebari::tree::Unversioned::tree(name))?.compact()?;
        }
        Ok(())
    }


    pub fn open_resource<R>(&self) -> Result<Tree<R::Key, R>>
    where
        R: Resource,
    {
        Ok(Tree::new(self.inner.tree(nebari::tree::Unversioned::tree(R::NAME))?))
    }

    fn open_index<R, F>(&self) -> Result<Tree<R::Key, F::Key>>
    where
        R: Resource,
        F: Resource,
    {
        let name = format!("{}_{}_index", R::NAME, F::NAME);

        Ok(Tree::new(self.inner.tree(nebari::tree::Unversioned::tree(name))?))
    }

    pub fn open_relation<R, F>(&self) -> Result<Relation<R::Key, R, F::Key, F>>
    where
        R: Resource,
        F: Resource,
    {
        Ok(Relation {
            local: self.open_resource()?,
            index: self.open_index::<R, F>()?,
            foreign: self.open_resource()?,
        })
    }
}
