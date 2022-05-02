use crate::{
    error::{Error, Result},
    primitives::{Database, OpenCollection, Relation, Tree},
    resource::Resource,
    Traverse,
};

pub struct Root<'a, Resource, Collection> {
    pub(super) db: &'a Database,
    pub(super) collection: Collection,
    pub(super) _resource: std::marker::PhantomData<Resource>,
}

impl<Resource, Collection> std::ops::Deref for Root<'_, Resource, Collection> {
    type Target = Collection;

    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}

impl<'a, Current> Root<'a, Current, Tree<Current::Key, Current>>
where
    Current: Resource,
{
    pub fn traverse<Target: Resource>(
        &self,
    ) -> Result<Root<'a, Target, <Current as Traverse<Target>>::Collection>>
    where
        Current: Traverse<Target>,
        Database: OpenCollection<Current::Collection>,
    {
        let collection: <Current as Traverse<Target>>::Collection = self.db.open_collection()?;

        Ok(Root {
            db: self.db,
            collection,
            _resource: Default::default(),
        })
    }
}

impl<'a, Current, Previous>
    Root<'a, Current, Relation<Current::Key, Current, Previous::Key, Previous>>
where
    Current: Resource,
    Previous: Resource,
{
    pub fn traverse<Target: Resource>(
        &self,
        key: &Current::Key,
    ) -> Result<Root<'a, Target, <Current as Traverse<Target>>::Collection>>
    where
        Current: Traverse<Target>,
        Database: OpenCollection<Current::Collection>,
    {
        let collection: <Current as Traverse<Target>>::Collection = self.db.open_collection()?;

        self.collection
            .contains_key(key)?
            .then(|| Root {
                db: self.db,
                collection,
                _resource: Default::default(),
            })
            .ok_or(Error::ForeignKeyConstraint)
    }
}
