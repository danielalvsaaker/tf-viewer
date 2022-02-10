use super::{Tree, Relation, Value, error::Result, query::{ActivityQuery, Key, GearQuery}};
use tf_models::{
    gear::Gear,
    activity::{Activity, Session, Record, Lap},
};

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