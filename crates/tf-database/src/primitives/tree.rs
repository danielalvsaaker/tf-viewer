use super::{Key, Value};
use crate::error::Result;

pub type Inner = nebari::Tree<nebari::tree::Unversioned, nebari::io::fs::StdFile>;

#[derive(Clone)]
pub struct Tree<K, V> {
    //pub inner: sled::Tree,
    pub inner: Inner,
    _type: std::marker::PhantomData<(K, V)>,
}

/*
impl<K, V> AsRef<sled::Tree> for Tree<K, V> {
    fn as_ref(&self) -> &sled::Tree {
        &self.inner
    }
}
*/

impl<K, V> AsRef<Inner> for Tree<K, V> {
    fn as_ref(&self) -> &Inner {
        &self.inner
    }
}

impl<K, V> Tree<K, V> {
    //pub fn new(tree: sled::Tree) -> Self {
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
        //self.inner.insert(key.as_key(), value.as_bytes()?)?;
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
        //Ok(self.inner.contains_key(&key.as_key())?)
        self.get(key).map(|x| x.is_some())
    }

    pub fn iter(&self, skip: usize, take: usize) -> impl Iterator<Item = K> {
        /*
        self.inner
            .iter()
            .keys()
            //.rev()
            .flatten()
            .flat_map(|x| K::from_bytes(&x))
            */
        let mut output = Vec::with_capacity(take);
        let mut keys_scanned = 0;
        self.inner.scan::<std::convert::Infallible, _, _, _, _>(
            &(..),
            false,
            |_, _, _| nebari::tree::ScanEvaluation::ReadData,
            |key, _| {
                if keys_scanned >= skip {
                    output.push(K::from_bytes(&key).unwrap());
                }

                keys_scanned += 1;
                if output.len() >= take {
                    nebari::tree::ScanEvaluation::Stop
                } else {
                    nebari::tree::ScanEvaluation::Skip
                }
            },
            |_, _, _| unreachable!(),
        ).unwrap();

        output.into_iter()
    }

    pub fn keys<L: Key>(&self, key: &L, skip: usize, take: usize) -> impl Iterator<Item = K> {
        /*
        self.inner
            .scan_prefix(key.as_prefix())
            .keys()
            //.rev()
            .flatten()
            .flat_map(|x| K::from_bytes(&x))
            */
        fn next_byte_sequence(start: &[u8]) -> Option<Vec<u8>> {
            let mut end = start.to_vec();
            // Modify the last byte by adding one. If it would wrap, we proceed to the
            // next byte.
            while let Some(last_byte) = end.pop() {
                if let Some(next) = last_byte.checked_add(1) {
                    end.push(next);
                    return Some(end);
                }
            }

            None
        }

        let key = key.as_prefix().to_vec();
        let next = next_byte_sequence(&key).map(|x| x.to_vec());
        let range = if let Some(ref next) = next {
            (std::ops::Bound::Included(key.as_slice()), std::ops::Bound::Excluded(next.as_slice()))
        } else {
            (std::ops::Bound::Included(key.as_slice()), std::ops::Bound::Unbounded)
        };
        
        let mut output = Vec::with_capacity(take);
        let mut keys_scanned = 0;
        self.inner.scan::<std::convert::Infallible, _, _, _, _>(
            &range,
            false,
            |_, _, _| nebari::tree::ScanEvaluation::ReadData,
            |key, _| {
                if keys_scanned > skip {
                    output.push(K::from_bytes(&key).unwrap());
                }

                keys_scanned += 1;
                if output.len() >= take {
                    nebari::tree::ScanEvaluation::Stop
                } else {
                    nebari::tree::ScanEvaluation::Skip
                }
            },
            |_, _, _| unreachable!(),
        ).unwrap();

        output.into_iter()
    }

    /*
    pub fn values<L: Key>(&self, key: &L) -> Result<impl Iterator<Item = V>> {
        Ok(self
            .inner
            .scan_prefix(key.as_prefix())
            .values()
            //.rev()
            .flatten()
            .flat_map(|x| V::from_bytes(&x)))
    }

    pub fn prev(&self, key: &K) -> Result<Option<K>> {
        let set: std::collections::BTreeSet<_> = self
            .inner
            .scan_prefix(key.as_prefix())
            .keys()
            .flatten()
            .collect();

        set.range::<[u8], _>((
            std::ops::Bound::Unbounded,
            std::ops::Bound::Excluded(key.as_key().as_slice()),
        ))
        .next_back()
        .map(AsRef::as_ref)
        .map(K::from_bytes)
        .transpose()
    }

    pub fn next(&self, key: &K) -> Result<Option<K>> {
        let set: std::collections::BTreeSet<_> = self
            .inner
            .scan_prefix(key.as_prefix())
            .keys()
            .flatten()
            .collect();

        set.range::<[u8], _>((
            std::ops::Bound::Excluded(key.as_key().as_slice()),
            std::ops::Bound::Unbounded,
        ))
        .next()
        .map(AsRef::as_ref)
        .map(K::from_bytes)
        .transpose()
    }
    */
}
