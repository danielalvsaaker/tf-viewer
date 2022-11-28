use crate::{
    error::Result,
    primitives::{
        collection::{Index, Iter, Tree},
        Key, Value,
    },
};

#[derive(Clone)]
pub struct Relation<LK, LV, FK, FV> {
    pub local: Tree<LK, LV>,
    pub index: Index<LK, LV, FK, FV>,
}

impl<LK, LV, FK, FV> Relation<LK, LV, FK, FV>
where
    LK: Key,
    LV: Value,
    FK: Key,
    FV: Value,
{
    pub fn get_foreign(&self, key: &LK) -> Result<Option<FK>> {
        self.local
            .get(key)?
            .and(self.index.key(key).transpose())
            .transpose()
    }

    pub fn get(&self, key: &LK) -> Result<Option<LV>> {
        self.index
            .contains_key(key)?
            .then(|| self.local.get(key).transpose())
            .flatten()
            .transpose()
    }

    pub fn keys<L: Key>(
        &self,
        key: &L,
        skip: usize,
        take: usize,
        reverse: bool,
    ) -> Result<Iter<LK>> {
        self.index.keys(key, skip, take, reverse)
    }

    pub fn join<L: Key>(
        &self,
        key: &L,
        skip: usize,
        take: usize,
        reverse: bool,
    ) -> Result<Iter<LK>> {
        self.index.join(key, skip, take, reverse)
    }

    pub fn contains_key(&self, key: &LK) -> Result<bool> {
        Ok(self.index.contains_key(key)? && self.local.contains_key(key)?)
    }

    pub fn insert(&self, key: &LK, value: &LV, foreign_key: &FK) -> Result<()> {
        self.index.insert(key, foreign_key)?;
        self.local.insert(key, value)?;

        Ok(())
    }

    pub fn link(&self, key: &LK, foreign_key: &FK) -> Result<()> {
        if self.local.contains_key(key)? {
            self.index.insert(key, foreign_key)?;
        }

        Ok(())
    }

    pub fn unlink(&self, key: &LK) -> Result<()> {
        self.index.remove(key)?;

        Ok(())
    }

    pub fn remove(&self, key: &LK) -> Result<Option<LV>> {
        self.index.remove(key)?;
        self.local.remove(key)
    }
}
