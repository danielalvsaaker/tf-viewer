use crate::{
    error::{Error, Result},
    primitives::{Database, OpenCollection, Relation, Tree},
    resource::Resource,
    Traverse,
};

pub struct Root<'a, T, C> {
    pub(super) _resource: std::marker::PhantomData<T>,
    pub(super) _db: &'a Database,
    pub(super) collection: C,
}

impl<'a, T> Root<'a, T, Tree<T::Key, T>>
where
    T: Resource,
{
    pub fn traverse<R: Resource>(&self) -> Result<Root<'a, R, T::Collection>>
    where
        T: Traverse<R>,
        Database: OpenCollection<T::Collection>,
    {
        let collection: T::Collection = OpenCollection::open_collection(self._db)?;

        Ok(Root {
            _resource: Default::default(),
            _db: self._db,
            collection,
        })
    }
}

impl<T, C> std::ops::Deref for Root<'_, T, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}

impl<'a, T, B> Root<'a, T, Relation<T::Key, T, B::Key, B>>
where
    B: Resource,
    T: Resource,
{
    pub fn traverse<R: Resource>(&self, key: &'a T::Key) -> Result<Root<'a, R, T::Collection>>
    where
        T: Traverse<R>,
        Database: OpenCollection<T::Collection>,
    {
        let collection: T::Collection = self._db.open_collection()?;

        self.collection
            .contains_key(key)?
            .then(|| Root {
                _resource: Default::default(),
                _db: self._db,
                collection,
            })
            .ok_or(Error::ForeignKeyConstraint)
    }
}
