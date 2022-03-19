use super::{Key, Tree, Value};
use crate::error::{Error, Result};
use sled::{
    transaction::{ConflictableTransactionError, ConflictableTransactionResult},
    Transactional,
};

#[derive(Clone)]
pub struct Relation<LK, LV, FK, FV> {
    pub local: Tree<LK, LV>,
    pub index: Tree<LK, FK>,
    pub foreign: Tree<FK, FV>,
}

impl<LK, LV, FK, FV> Relation<LK, LV, FK, FV>
where
    LK: Key,
    LV: Value,
    FK: Key,
    FV: Value,
{
    pub fn get_foreign(&self, key: &LK) -> Result<Option<FK>> {
        /*
        (
            self.local.as_ref(),
            self.index.as_ref(),
            self.foreign.as_ref(),
        )
            .transaction(
                |(local, index, foreign)| -> ConflictableTransactionResult<_, Error> {
                    let key = key.as_key();

                    Ok(local
                        .get(&key)?
                        .and(index.get(&key).transpose())
                        .transpose()?
                        .filter(|key| foreign.get(key).map(|x| x.is_some()).unwrap_or_default()))
                },
            )?
            .map(|x| FK::from_bytes(&x))
            .transpose()
            */
        let key = key.as_key();
        self.local.as_ref().get(&key)?
            .and(self.index.as_ref().get(&key).transpose())
            .transpose()?
            .filter(|key| self.foreign.as_ref().get(&key).map(|x| x.is_some()).unwrap_or_default())
            .map(|x| FK::from_bytes(&x))
            .transpose()
    }

    pub fn get(&self, key: &LK) -> Result<Option<LV>> {
        /*
        (
            self.local.as_ref(),
            self.index.as_ref(),
            self.foreign.as_ref(),
        )
            .transaction(
                |(local, index, foreign)| -> ConflictableTransactionResult<_, Error> {
                    let key = key.as_key();

                    Ok(local.get(&key)?.filter(|_| {
                        if let Ok(Some(foreign_key)) = index.get(key) {
                            foreign
                                .get(foreign_key)
                                .map(|x| x.is_some())
                                .unwrap_or_default()
                        } else {
                            false
                        }
                    }))
                },
            )?
            .map(|x| LV::from_bytes(&x))
            .transpose()
            */
        let key = key.as_key();
        self.local.as_ref().get(&key)?.filter(|_| {
            if let Ok(Some(foreign_key)) = self.index.as_ref().get(&key) {
                self.foreign
                    .as_ref()
                    .get(&foreign_key)
                    .map(|x| x.is_some())
                    .unwrap_or_default()
            } else {
                false
            }
        })
        .map(|x| LV::from_bytes(&x))
        .transpose()
    }

    pub fn keys<L: Key>(&self, key: &L, skip: usize, take: usize) -> impl Iterator<Item = LK> {
        self.local.keys(key, skip, take)
    }

    /*
    pub fn values<'a, L: Key>(&'a self, key: &'a L) -> impl Iterator<Item = FK> + 'a
    where
        LK: 'a,
        LV: 'a,
        FK: 'a,
        FV: 'a,
    {
        self.local
            .keys(key)
            .flat_map(|key| self.get_foreign(&key))
            .flatten()
    }
    */

    pub fn contains_key(&self, key: &LK) -> Result<bool> {
        self.get(key).map(|v| v.is_some())
    }

    pub fn insert(&self, key: &LK, value: &LV, foreign_key: &FK) -> Result<()> {
        /*
        Ok((
            self.local.as_ref(),
            self.index.as_ref(),
            self.foreign.as_ref(),
        )
            .transaction(|(local, index, foreign)| {
                if foreign.get(foreign_key.as_key())?.is_some() {
                    let value = value
                        .as_bytes()
                        .map_err(ConflictableTransactionError::Abort)?;

                    index.insert(key.as_key(), foreign_key.as_key())?;
                    local.insert(key.as_key(), value)?;

                    Ok(())
                } else {
                    Err(ConflictableTransactionError::Abort(
                        Error::ForeignKeyConstraint,
                    ))
                }
            })?)
            */
        self.local.as_ref().set(key.as_key(), value.as_bytes().unwrap())?;
        self.index.as_ref().set(key.as_key(), foreign_key.as_key())?;

        Ok(())
    }

    pub fn remove(&self, key: &LK) -> Result<Option<()>> {
        //Ok(self.index.as_ref().remove(&key.as_key())?.map(|_| ()))
        Ok(todo!())
    }
}
