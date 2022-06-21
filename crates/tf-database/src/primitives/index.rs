use super::{Key, Tree, Value};
use crate::error::{Error, Result};

#[derive(Clone)]
pub struct Index<LK, LV, FK, FV> {
    pub index: Tree<LK, FK>,
    pub foreign: Tree<FK, FV>,
    _type: std::marker::PhantomData<LV>,
}

impl<LK, LV, FK, FV> Index<LK, LV, FK, FV> {
    pub fn new(index: Tree<LK, FK>, foreign: Tree<FK, FV>) -> Self {
        Self {
            index,
            foreign,
            _type: Default::default(),
        }
    }
}

impl<LK, LV, FK, FV> Index<LK, LV, FK, FV>
where
    LK: Key,
    FK: Key,
    FV: Value,
{
    pub fn key(&self, key: &LK) -> Result<Option<FK>> {
        self.index
            .as_ref()
            .get(&key.as_key())?
            .filter(|foreign_key| {
                self.foreign
                    .as_ref()
                    .get(foreign_key)
                    .map(|x| x.is_some())
                    .unwrap_or_default()
            })
            .map(|x| FK::from_bytes(&x))
            .transpose()
    }

    pub fn contains_key(&self, key: &LK) -> Result<bool> {
        self.key(key).map(|x| x.is_some())
    }

    pub fn get(&self, key: &LK) -> Result<Option<FV>> {
        let foreign_key = self.key(key)?;

        foreign_key
            .and_then(|x| self.foreign.get(&x).transpose())
            .transpose()
    }

    pub fn insert(&self, key: &LK, foreign_key: &FK) -> Result<()> {
        let foreign_key = foreign_key.as_key();

        if self.foreign.as_ref().get(&foreign_key)?.is_some() {
            self.index.as_ref().set(key.as_key(), foreign_key)?;
            Ok(())
        } else {
            Err(Error::ForeignKeyConstraint)
        }
    }

    pub fn remove(&self, key: &LK) -> Result<()> {
        self.index.as_ref().remove(&key.as_key())?;
        Ok(())
    }

    pub fn keys<L: Key>(
        &self,
        key: &L,
        skip: usize,
        take: usize,
        reverse: bool,
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
            reverse,
            |_, _, _| nebari::tree::ScanEvaluation::ReadData,
            |key, _| {
                keys_scanned += 1;

                if keys_scanned > skip {
                    output.push(LK::from_bytes(key));
                }

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

    pub fn count<L: Key>(&self, key: &L) -> Result<usize> {
        let prefix = key.as_prefix();

        let mut total = 0;

        self.index.as_ref().scan::<Error, _, _, _, _>(
            &(..),
            false,
            |_, _, _| nebari::tree::ScanEvaluation::ReadData,
            |key, _| {
                if key.starts_with(&prefix) {
                    total += 1;
                }

                nebari::tree::ScanEvaluation::Skip
            },
            |_, _, _| unreachable!(),
        )?;

        Ok(total)
    }
}
