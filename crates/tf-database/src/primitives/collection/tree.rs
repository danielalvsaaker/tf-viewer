use crate::{
    error::{Error, Result},
    primitives::{collection::Iter, ArcBytes, Key, Value},
};
use nebari::tree::ScanEvaluation;

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

    pub fn get_raw(&self, key: &K) -> Result<Option<ArcBytes<'static>>> {
        Ok(self.inner.get(&key.as_key())?)
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
        Ok(self.inner.get(&key.as_key())?.is_some())
    }

    pub fn iter(&self, skip: usize, take: usize, reverse: bool) -> Result<Iter<K>> {
        let mut output = Vec::with_capacity(take);
        let mut keys_scanned = 0_usize;

        self.inner.scan::<Error, _, _, _, _>(
            &(..),
            reverse,
            |_, _, _| ScanEvaluation::ReadData,
            |key, _| {
                keys_scanned += 1;

                if keys_scanned.saturating_sub(skip) > take {
                    return ScanEvaluation::Stop;
                }

                if keys_scanned > skip {
                    if let Ok(key) = K::from_bytes(key) {
                        output.push(key);
                    }
                }

                ScanEvaluation::Skip
            },
            |_, _, _| unreachable!(),
        )?;

        Ok(Iter {
            iter: output.into_iter(),
            total_count: self.as_ref().count() as usize,
        })
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
