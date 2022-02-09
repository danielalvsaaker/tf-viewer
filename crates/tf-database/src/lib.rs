/*pub mod activities;
pub mod gear;
pub mod users;

use anyhow::Result;

*/

use crate::query::{ActivityQuery, GearQuery};
use rmp_serde as rmps;
use tf_models::activity::{Activity, Lap, Record, Session};
use tf_models::gear::Gear;

use query::Key;
use serde::{de::DeserializeOwned, Serialize};
use sled::Transactional;

pub trait Value
where
    Self: Sized + Serialize + DeserializeOwned,
{
    fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(rmps::to_vec(self)?)
    }

    fn from_bytes(data: &[u8]) -> Result<Self> {
        Ok(rmps::from_read_ref(data)?)
    }
}

#[derive(Clone)]
pub struct Tree<K, V> {
    inner: sled::Tree,
    _type: std::marker::PhantomData<(K, V)>,
}

impl<K, V> AsRef<sled::Tree> for Tree<K, V> {
    fn as_ref(&self) -> &sled::Tree {
        &self.inner
    }
}

impl<K, V> Tree<K, V>
where
    K: Key,
    V: Value,
{
    pub fn get(&self, key: &K) -> Result<Option<V>> {
        self.inner
            .get(key.as_key())?
            .map(|x| V::from_bytes(&x))
            .transpose()
    }

    pub fn keys(&self, key: &K) -> Result<impl Iterator<Item = K>> {
        Ok(self
            .inner
            .scan_prefix(key.as_prefix())
            .keys()
            .rev()
            .flatten()
            .flat_map(|x| K::from_bytes(&x)))
    }

    pub fn values(&self, key: &K) -> Result<impl Iterator<Item = V>> {
        Ok(self
            .inner
            .scan_prefix(key.as_prefix())
            .values()
            .rev()
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

        Ok(set
            .range::<[u8], _>((
                std::ops::Bound::Unbounded,
                std::ops::Bound::Excluded(key.as_key().as_slice()),
            ))
            .next_back()
            .map(AsRef::as_ref)
            .and_then(K::from_bytes))
    }

    pub fn next(&self, key: &K) -> Result<Option<K>> {
        let set: std::collections::BTreeSet<_> = self
            .inner
            .scan_prefix(key.as_prefix())
            .keys()
            .flatten()
            .collect();

        Ok(set
            .range::<[u8], _>((
                std::ops::Bound::Excluded(key.as_key().as_slice()),
                std::ops::Bound::Unbounded,
            ))
            .next()
            .map(AsRef::as_ref)
            .and_then(K::from_bytes))
    }
}

impl<'a, K: 'a, V: 'a> Tree<K, V>
where
    K: Key + From<&'a V>,
    V: Value,
{
    pub fn insert(&self, value: &'a V) -> Result<Option<()>> {
        let key = K::from(value);

        Ok(self
            .inner
            .insert(key.as_key(), value.as_bytes()?)?
            .map(|_| ()))
    }
}

#[derive(Clone)]
pub struct Relation<K, F, V> {
    index: Tree<K, F>,
    foreign: Tree<F, V>,
}

impl<K, F, V> Relation<K, F, V>
where
    K: Key,
    F: Key,
    V: Value,
{
    pub fn get(&self, key: &K) -> Result<Option<V>> {
        let res =
            (self.index.as_ref(), self.foreign.as_ref()).transaction(|(index, foreign)| {
                let key = index.get(&key.as_key())?;
                let key = key.and_then(|x| foreign.get(x).transpose()).transpose()?;
                Ok::<_, sled::transaction::ConflictableTransactionError<sled::Error>>(key)
            })?;

        res.map(|x| V::from_bytes(&x)).transpose()
    }

    pub fn insert(&self, key: &K, foreign_key: &F) -> Result<Option<()>> {
        Ok((self.index.as_ref(), self.foreign.as_ref())
            .transaction(|(index, foreign)| {
                if foreign.get(foreign_key.as_key())?.is_some() {
                    Ok(index.insert(key.as_key(), foreign_key.as_key())?)
                } else {
                    Ok(None)
                }
            })?
            .map(|_| ()))
    }
}

impl<T> Value for T where T: Sized + Serialize + DeserializeOwned {}

pub trait Insert<T: Value> {
    fn insert(&self, value: T) -> Result<()>;
}

pub trait Get<T> {
    type Key;

    fn get(&self, key: &Self::Key) -> Result<Option<T>>;
}

/*
impl Insert<Activity> for Database {
    fn insert(&self, mut value: Activity) -> Result<()> {
        let key = &vec![];

        self.activity
            .usernameid_session
            .insert(key, value.session.as_bytes()?)?;
        self.activity
            .usernameid_record
            .insert(key, value.record.as_bytes()?)?;
        self.activity
            .usernameid_lap
            .insert(key, value.lap.as_bytes()?)?;

        self.activity
            .usernameid_activity
            .insert(key, value.as_bytes()?)?;

        Ok(())
    }
}

impl Insert<Session> for Database {
    fn insert(&self, value: Session) -> Result<()> {
        let key = &vec![];

        self.activity
            .usernameid_session
            .insert(key, value.as_bytes()?)?;

        Ok(())
    }
}

impl Get<Activity> for Database {
    type Key = ActivityQuery;

    fn get(&self, key: &Self::Key) -> Result<Option<Activity>> {
        todo!()
    }
}

user -> activity
     |       ^
     -> gear |
*/

/*
trait Relationship<T: Value> {
    type Key: Key;

    fn get(key: &Self::Key) -> Result<Option<T>>;
}
*/

/*
impl Relationship<Gear> for (ActivityTree, GearTree) {
    type Key = ActivityQuery;

    fn get(key: &Self::Key) -> Result<Option<Gear>> {

}
*/

#[derive(Clone)]
pub struct ActivityTree<'a> {
    pub session: Tree<ActivityQuery<'a>, Session>,
    pub record: Tree<ActivityQuery<'a>, Record>,
    pub lap: Tree<ActivityQuery<'a>, Vec<Lap>>,
    pub gear: Relation<ActivityQuery<'a>, GearQuery<'a>, Gear>,
}

impl ActivityTree<'_> {
    pub fn insert(&self, value: &Activity) -> Result<()> {
        let query = ActivityQuery::from(value);
        let key = query.as_key();

        self.session.inner.insert(&key, value.session.as_bytes()?)?;
        self.record.inner.insert(&key, value.record.as_bytes()?)?;
        self.lap.inner.insert(&key, value.lap.as_bytes()?)?;

        if let Some(gear_id) = &value.gear_id {
            let gear_query = GearQuery {
                user_id: std::borrow::Cow::Borrowed(&query.user_id),
                id: gear_id.into(),
            };

            self.gear.insert(&query, &gear_query)?;
        }

        Ok(())
    }

    pub fn get(&self, key: &ActivityQuery<'_>) -> Result<Option<Activity>> {
        let session = match self.session.get(key)? {
            Some(session) => session,
            None => return Ok(None),
        };

        let record = match self.record.get(key)? {
            Some(record) => record,
            None => return Ok(None),
        };

        let lap = match self.lap.get(key)? {
            Some(lap) => lap,
            None => return Ok(None),
        };

        let gear_id = self.gear.index.get(key)?.map(|x| x.id.into());

        Ok(Some(Activity {
            owner: key.user_id.to_string(),
            id: key.id.to_string(),
            gear_id,
            session,
            record,
            lap,
        }))
    }
}

#[derive(Clone)]
pub struct GearTree<'a> {
    pub gear: Tree<GearQuery<'a>, Gear>,
}

#[derive(Clone)]
pub struct Database<'a> {
    pub activity: ActivityTree<'a>,
    pub gear: GearTree<'a>,
    _db: sled::Db,
}

impl<'a> Database<'a> {
    pub fn load_or_create() -> Result<Self> {
        let db = sled::Config::new()
            .path("db")
            .use_compression(true)
            .open()?;

        let gear = Self::open_tree(&db, "gear")?;

        Ok(Self {
            activity: ActivityTree {
                session: Self::open_tree(&db, "activity_session")?,
                record: Self::open_tree(&db, "activity_record")?,
                lap: Self::open_tree(&db, "activity_lap")?,
                gear: Relation {
                    index: Self::open_tree(&db, "activity_gear")?,
                    foreign: gear.clone(),
                },
            },
            gear: GearTree {
                gear: Self::open_tree(&db, "gear_gear")?,
            },
            _db: db,
        })
    }

    fn open_tree<K, V>(db: &sled::Db, name: &'static str) -> Result<Tree<K, V>> {
        Ok(Tree {
            inner: db.open_tree(name)?,
            _type: Default::default(),
        })
    }
}

pub mod error;
pub mod query;
pub use error::Result;

/*
pub mod activity;
pub mod gear;
pub mod user;
*/
