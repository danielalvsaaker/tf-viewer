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

pub trait OpenCollection<'a, C> {
    fn open_collection(&'a self) -> Result<C>;
}

impl<'a, R> OpenCollection<'a, Tree<R::Key, R>> for Database
where
    R: Resource,
{
    fn open_collection(&'a self) -> Result<Tree<R::Key, R>> {
        self.open_resource()
    }
}

impl<'a, R, S> OpenCollection<'a, Relation<'a, R::Key, R, S::Key, S>> for Database
where
    R: Resource,
    S: Resource,
{
    fn open_collection(&'a self) -> Result<Relation<'a, R::Key, R, S::Key, S>> {
        self.open_relation()
    }
}

impl Database {
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        Ok(Self {
            inner: nebari::Config::default_for(path).vault(LZ4Vault).open()?,
        })
    }

    pub fn compact(&self) -> Result<()> {
        for name in self.inner.tree_names()? {
            self.inner
                .tree(nebari::tree::Unversioned::tree(name))?
                .compact()?;
        }
        Ok(())
    }

    pub fn open_resource<R>(&self) -> Result<Tree<R::Key, R>>
    where
        R: Resource,
    {
        Ok(Tree::new(
            self.inner.tree(nebari::tree::Unversioned::tree(R::NAME))?,
        ))
    }

    fn open_index<R, F>(&self) -> Result<Tree<R::Key, F::Key>>
    where
        R: Resource,
        F: Resource,
    {
        let name = format!("{}_{}_index", R::NAME, F::NAME);

        Ok(Tree::new(
            self.inner.tree(nebari::tree::Unversioned::tree(name))?,
        ))
    }

    pub fn open_relation<R, F>(&self) -> Result<Relation<'_, R::Key, R, F::Key, F>>
    where
        R: Resource,
        F: Resource,
    {
        Ok(Relation {
            root: &self.inner,
            local: self.open_resource()?,
            index: self.open_index::<R, F>()?,
            foreign: self.open_resource()?,
        })
    }
}
