use super::{Key, Tree, Value};
use crate::error::{Error, Result};

#[derive(Clone)]
pub struct Relation<'a, LK, LV, FK, FV> {
    pub root: &'a super::Inner,
    pub local: Tree<LK, LV>,
    pub index: Tree<LK, FK>,
    pub foreign: Tree<FK, FV>,
}

impl<'a, LK, LV, FK, FV> Relation<'a, LK, LV, FK, FV>
where
    LK: Key,
    LV: Value,
    FK: Key,
    FV: Value,
{
    pub fn get_foreign(&self, key: &LK) -> Result<Option<FK>> {
        let key = key.as_key();
        self.local
            .as_ref()
            .get(&key)?
            .and(self.index.as_ref().get(&key).transpose())
            .transpose()?
            .filter(|key| {
                self.foreign
                    .as_ref()
                    .get(&key)
                    .map(|x| x.is_some())
                    .unwrap_or_default()
            })
            .map(|x| FK::from_bytes(&x))
            .transpose()
    }

    pub fn get(&self, key: &LK) -> Result<Option<LV>> {
        let key = key.as_key();
        self.local
            .as_ref()
            .get(&key)?
            .filter(|_| {
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

    pub fn keys<L: Key>(
        &self,
        key: &L,
        skip: usize,
        take: usize,
    ) -> Result<impl Iterator<Item = LK>> {
        let key = key.as_prefix().to_vec();
        let next = super::key::next_byte_sequence(&key).map(|x| x.to_vec());

        let range = if let Some(ref next) = next {
            (
                std::ops::Bound::Included(key.as_slice()),
                std::ops::Bound::Excluded(next.as_slice()),
            )
        } else {
            (
                std::ops::Bound::Included(key.as_slice()),
                std::ops::Bound::Unbounded,
            )
        };

        let mut output = Vec::with_capacity(take);
        let mut keys_scanned = 0;

        self.index.as_ref().scan::<Error, _, _, _, _>(
            &range,
            false,
            |_, _, _| nebari::tree::ScanEvaluation::ReadData,
            |key, _| {
                if keys_scanned > skip {
                    output.push(LK::from_bytes(key));
                }

                keys_scanned += 1;
                if output.len() >= take {
                    nebari::tree::ScanEvaluation::Stop
                } else {
                    nebari::tree::ScanEvaluation::Skip
                }
            },
            |_, _, _| unreachable!(),
        )?;

        Ok(output.into_iter().flatten())
    }

    pub fn values<L: Key>(
        &self,
        key: &L,
        skip: usize,
        take: usize,
    ) -> Result<impl Iterator<Item = FK>> {
        let key = key.as_prefix().to_vec();
        let next = super::key::next_byte_sequence(&key).map(|x| x.to_vec());

        let range = if let Some(ref next) = next {
            (
                std::ops::Bound::Included(key.as_slice()),
                std::ops::Bound::Excluded(next.as_slice()),
            )
        } else {
            (
                std::ops::Bound::Included(key.as_slice()),
                std::ops::Bound::Unbounded,
            )
        };

        let mut output = Vec::with_capacity(take);
        let mut keys_scanned = 0;

        self.local.as_ref().scan::<Error, _, _, _, _>(
            &range,
            false,
            |_, _, _| nebari::tree::ScanEvaluation::ReadData,
            |key, _| {
                if keys_scanned > skip {
                    if let Ok(Some(foreign_key)) = self.index.as_ref().get(key) {
                        output.push(foreign_key);
                    }
                }

                keys_scanned += 1;
                if output.len() >= take {
                    nebari::tree::ScanEvaluation::Stop
                } else {
                    nebari::tree::ScanEvaluation::Skip
                }
            },
            |_, _, _| unreachable!(),
        )?;

        Ok(output.into_iter().flat_map(|key| FK::from_bytes(&key)))
    }

    pub fn contains_key(&self, key: &LK) -> Result<bool> {
        self.get(key).map(|v| v.is_some())
    }

    pub fn insert(&self, key: &LK, value: &LV, foreign_key: &FK) -> Result<()> {
        let foreign_key = foreign_key.as_key();

        let transaction = self.root.transaction::<_, super::tree::Inner>(&[
            self.local.as_ref(),
            self.index.as_ref(),
            self.foreign.as_ref(),
        ])?;
        {
            let (mut local, mut index, mut foreign) = (
                transaction.tree::<nebari::tree::Unversioned>(0).unwrap(),
                transaction.tree::<nebari::tree::Unversioned>(1).unwrap(),
                transaction.tree::<nebari::tree::Unversioned>(2).unwrap(),
            );

            if foreign.get(&foreign_key)?.is_some() {
                local.set(key.as_key(), value.as_bytes()?)?;
                index.set(key.as_key(), foreign_key)?;
                Ok(())
            } else {
                Err(Error::ForeignKeyConstraint)
            }
        }
        .and_then(|_| Ok(transaction.commit()?))
    }

    pub fn remove(&self, key: &LK) -> Result<()> {
        let key = key.as_key();

        let transaction = self
            .root
            .transaction::<_, super::tree::Inner>(&[self.local.as_ref(), self.index.as_ref()])?;
        {
            let (mut local, mut index) = (
                transaction.tree::<nebari::tree::Unversioned>(0).unwrap(),
                transaction.tree::<nebari::tree::Unversioned>(1).unwrap(),
            );

            local.remove(&key)?;
            index.remove(&key)?;

            Ok(())
        }
        .and_then(|_| Ok(transaction.commit()?))
    }
}
