mod index;
mod relation;
mod tree;

mod key;
mod value;

pub use index::Index;
pub use key::Key;
pub use relation::Relation;
pub use tree::Tree;
pub use value::Value;

use crate::{error::Result, Resource};
use nebari::tree::Root;
pub use nebari::ArcBytes;

pub type Inner = nebari::Roots<nebari::io::fs::StdFile>;

#[derive(Clone, Copy, Debug)]
struct LZ4Vault;

impl nebari::Vault for LZ4Vault {
    type Error = lz4_flex::block::DecompressError;

    fn encrypt(&self, payload: &[u8]) -> Result<Vec<u8>, Self::Error> {
        Ok(lz4_flex::compress_prepend_size(payload))
    }

    fn decrypt(&self, payload: &[u8]) -> Result<Vec<u8>, Self::Error> {
        lz4_flex::decompress_size_prepended(payload)
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

impl<L, F> OpenCollection<Relation<L::Key, L, F::Key, F>> for Database
where
    L: Resource,
    F: Resource,
{
    fn open_collection(&self) -> Result<Relation<L::Key, L, F::Key, F>> {
        self.open_relation()
    }
}

impl<L, F> OpenCollection<Index<L::Key, L, F::Key, F>> for Database
where
    L: Resource,
    F: Resource,
{
    fn open_collection(&self) -> Result<Index<L::Key, L, F::Key, F>> {
        self.open_index()
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

    fn open_index<L, F>(&self) -> Result<Index<L::Key, L, F::Key, F>>
    where
        L: Resource,
        F: Resource,
    {
        let name = format!("{}_{}_index", L::NAME, F::NAME);

        Ok(Index::new(
            Tree::new(self.inner.tree(nebari::tree::Unversioned::tree(name))?),
            self.open_resource()?,
        ))
    }

    pub fn open_relation<L, F>(&self) -> Result<Relation<L::Key, L, F::Key, F>>
    where
        L: Resource,
        F: Resource,
    {
        Ok(Relation {
            local: self.open_resource()?,
            index: self.open_index::<L, F>()?,
        })
    }
}
