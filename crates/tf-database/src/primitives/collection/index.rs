use crate::{
    error::{Error, Result},
    primitives::{
        collection::{Iter, Tree},
        Key, Value,
    },
};
use nebari::tree::ScanEvaluation;

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
    ) -> Result<Iter<LK>> {
        let prefix = key.as_prefix();
        let next = super::next_byte_sequence(&prefix);

        let range = if let Some(ref next) = next {
            (
                std::ops::Bound::Included(prefix.as_slice()),
                std::ops::Bound::Excluded(next.as_slice()),
            )
        } else {
            (
                std::ops::Bound::Included(prefix.as_slice()),
                std::ops::Bound::Unbounded,
            )
        };

        let mut output = Vec::with_capacity(take);
        let mut keys_scanned = 0_usize;

        self.index.as_ref().scan::<Error, _, _, _, _>(
            &range,
            reverse,
            |_, _, _| ScanEvaluation::ReadData,
            |_, _| ScanEvaluation::ReadData,
            |local_key, _, foreign_key| {
                let foreign_key =
                    FK::from_bytes(&foreign_key).map_err(nebari::AbortError::Other)?;

                if self
                    .foreign
                    .contains_key(&foreign_key)
                    .map_err(nebari::AbortError::Other)?
                {
                    keys_scanned += 1;

                    if keys_scanned <= skip || keys_scanned.saturating_sub(skip) > take {
                        return Ok(());
                    }

                    if let Ok(local_key) = LK::from_bytes(&local_key) {
                        output.push(local_key);
                    }
                }

                Ok(())
            },
        )?;

        Ok(Iter {
            iter: output.into_iter(),
            total_count: keys_scanned,
        })
    }

    pub fn join<L: Key>(
        &self,
        key: &L,
        skip: usize,
        take: usize,
        reverse: bool,
    ) -> Result<Iter<LK>> {
        let prefix = key.as_prefix();
        let key = key.as_key();
        let next = super::next_byte_sequence(&prefix);

        let range = if let Some(ref next) = next {
            (
                std::ops::Bound::Included(prefix.as_slice()),
                std::ops::Bound::Excluded(next.as_slice()),
            )
        } else {
            (
                std::ops::Bound::Included(prefix.as_slice()),
                std::ops::Bound::Unbounded,
            )
        };

        let mut output = Vec::with_capacity(take);
        let mut keys_scanned = 0_usize;

        self.index.as_ref().scan::<Error, _, _, _, _>(
            &range,
            reverse,
            |_, _, _| ScanEvaluation::ReadData,
            |_, _| ScanEvaluation::ReadData,
            |local_key, _, foreign_key| {
                if key == foreign_key.as_slice()
                    && self
                        .foreign
                        .as_ref()
                        .get(&foreign_key)
                        .map(|x| x.is_some())?
                {
                    keys_scanned += 1;

                    if keys_scanned <= skip || keys_scanned.saturating_sub(skip) > take {
                        return Ok(());
                    }

                    if let Ok(local_key) = LK::from_bytes(&local_key) {
                        output.push(local_key);
                    }
                }

                Ok(())
            },
        )?;

        Ok(Iter {
            iter: output.into_iter(),
            total_count: keys_scanned,
        })
    }
}
