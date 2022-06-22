use super::{Key, Value};
use crate::error::{Error, Result};

pub type Inner = nebari::Tree<nebari::tree::Unversioned, nebari::io::fs::StdFile>;

#[derive(Clone)]
pub struct Tree<K, V> {
    pub inner: Inner,
    _type: std::marker::PhantomData<(K, V)>,
}

impl<K, V> AsRef<Inner> for Tree<K, V> {
    fn as_ref(&self) -> &Inner {
        &self.inner
    }
}

impl<K, V> Tree<K, V> {
    pub fn new(tree: Inner) -> Self {
        Self {
            inner: tree,
            _type: Default::default(),
        }
    }
}

impl<K, V> Tree<K, V>
where
    K: Key,
    V: Value,
{
    pub fn get(&self, key: &K) -> Result<Option<V>> {
        self.inner
            .get(&key.as_key())?
            .map(|x| V::from_bytes(&x))
            .transpose()
    }

    pub fn insert(&self, key: &K, value: &V) -> Result<()> {
        self.inner.set(key.as_key(), value.as_bytes()?)?;

        Ok(())
    }

    pub fn remove(&self, key: &K) -> Result<Option<V>> {
        self.inner
            .remove(&key.as_key())?
            .map(|x| V::from_bytes(&x))
            .transpose()
    }

    pub fn contains_key(&self, key: &K) -> Result<bool> {
        self.get(key).map(|x| x.is_some())
    }

    pub fn iter(&self, skip: usize, take: usize, reverse: bool) -> Result<impl Iterator<Item = K>> {
        let mut output = Vec::with_capacity(take);
        let mut keys_scanned = 0;

        self.inner.scan::<Error, _, _, _, _>(
            &(..),
            reverse,
            |_, _, _| nebari::tree::ScanEvaluation::ReadData,
            |key, _| {
                if keys_scanned >= skip {
                    output.push(K::from_bytes(key).unwrap());
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

        Ok(output.into_iter())
    }

    pub fn keys<L: Key>(
        &self,
        key: &L,
        skip: usize,
        take: usize,
    ) -> Result<impl Iterator<Item = K>> {
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

        self.inner.scan::<Error, _, _, _, _>(
            &range,
            false,
            |_, _, _| nebari::tree::ScanEvaluation::ReadData,
            |key, _| {
                if keys_scanned > skip {
                    output.push(K::from_bytes(key));
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

    pub fn count(&self) -> Result<usize> {
        Ok(self.inner.count() as usize)
    }

    pub fn prev(&self, key: &K) -> Result<Option<K>> {
        let mut output = Ok(None);

        self.inner.scan::<Error, _, _, _, _>(
            &(
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(key.as_key().as_slice()),
            ),
            false,
            |_, _, _| nebari::tree::ScanEvaluation::ReadData,
            |k, _| {
                if k.starts_with(&key.as_prefix()) {
                    output = Some(K::from_bytes(k)).transpose();
                }

                nebari::tree::ScanEvaluation::Stop
            },
            |_, _, _| unreachable!(),
        )?;

        output
    }

    pub fn next(&self, key: &K) -> Result<Option<K>> {
        let mut output = Ok(None);

        self.inner.scan::<Error, _, _, _, _>(
            &(
                std::ops::Bound::Excluded(key.as_key().as_slice()),
                std::ops::Bound::Unbounded,
            ),
            true,
            |_, _, _| nebari::tree::ScanEvaluation::ReadData,
            |k, _| {
                if k.starts_with(&key.as_prefix()) {
                    output = Some(K::from_bytes(k)).transpose();
                }

                nebari::tree::ScanEvaluation::Stop
            },
            |_, _, _| unreachable!(),
        )?;

        output
    }
}
