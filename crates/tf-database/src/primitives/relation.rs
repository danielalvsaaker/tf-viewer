use super::{Index, Key, Tree, Value};
use crate::error::Result;

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
    ) -> Result<impl Iterator<Item = LK>> {
        self.index.keys(key, skip, take, reverse)
    }

    pub fn count<L: Key>(&self, key: &L) -> Result<usize> {
        self.index.count(key)
    }

    /*
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
    */

    pub fn contains_key(&self, key: &LK) -> Result<bool> {
        self.get(key).map(|v| v.is_some())
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
