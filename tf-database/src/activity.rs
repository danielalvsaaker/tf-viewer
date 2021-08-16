use crate::query::{ActivityQuery, UserQuery};
use rmp_serde as rmps;
use tf_models::{
    backend::{Lap, Record, Session},
    Activity,
};

use crate::Result;

#[derive(Clone)]
pub struct ActivityTree {
    //pub(super) usernameid_meta: sled::Tree,
    pub(super) usernameid_session: sled::Tree,
    pub(super) usernameid_record: sled::Tree,
    pub(super) usernameid_lap: sled::Tree,

    // Indices
    pub(super) usernameid_gearid: sled::Tree,
}

impl ActivityTree {
    /*pub fn exists<Q: Query>(&self, query: &Q) -> Result<bool> {
            Ok(self.usernameid_session.contains_key(&query.to_key())?)
        }

        pub fn insert(&self, activity: Activity) -> Result<()> {
            let key = ActivityQuery::from(activity).to_key();

            let session = rmps::to_vec(&activity.session)?;
            self.usernameid_session.insert(&key, session)?;

            let record = rmps::to_vec(&activity.record)?;
            self.usernameid_record.insert(&key, record)?;

            let lap = rmps::to_vec(&activity.lap)?;
            self.usernameid_lap.insert(&key, lap)?;

            let gear_id = rmps::to_vec(&activity.gear_id)?;
            self.usernameid_gearid.insert(&key, gear_id)?;

            Ok(())
        }

        pub fn username_iter_meta(
            &self,
            user: &UserQuery,
        ) -> Result<impl Iterator<Item = ActivityMeta>> {
            Ok(self
                .usernameid_gearid
                .scan_prefix(&user.to_prefix())
                .values()
                .rev()
                .flatten()
                .flat_map(|x| rmps::from_read_ref(&x)))
        }


        pub fn username_iter_id(&self, user: &UserQuery) -> Result<impl Iterator<Item = String>> {
            Ok(self
                .usernameid_session
                .scan_prefix(&user.to_prefix())
                .keys()
                .rev()
                .flatten()
                .map(|x| x.split(|y| y == &0xff).last().unwrap().to_vec())
                .flat_map(String::from_utf8))
        }

        pub fn iter_username(&self) -> Result<impl Iterator<Item = String>> {
            Ok(self
                .usernameid_session
                .iter()
                .keys()
                .rev()
                .flatten()
                .map(|x| x.split(|y| y == &0xff).next().unwrap().to_vec())
                .flat_map(String::from_utf8))
        }

        pub fn iter_id(&self) -> Result<impl Iterator<Item = String>> {
            Ok(self
                .usernameid_session
                .iter()
                .keys()
                .rev()
                .flatten()
                .map(|x| x.split(|y| y == &0xff).last().unwrap().to_vec())
                .flat_map(String::from_utf8))
        }
    */
    pub fn username_iter_id(&self, query: &UserQuery) -> Result<impl Iterator<Item = String>> {
        Ok(self
            .usernameid_session
            .scan_prefix(&query.to_prefix())
            .keys()
            .rev()
            .flatten()
            .map(|x| x.split(|y| y == &0xff).last().unwrap().to_vec())
            .flat_map(String::from_utf8))
    }

    pub fn insert_activity(&self, query: &UserQuery, activity: &Activity) -> Result<()> {
        let mut key = query.to_prefix();
        key.extend_from_slice(&activity.id.as_bytes());

        let session = rmps::to_vec(&activity.session)?;
        self.usernameid_session.insert(&key, session)?;

        let record = rmps::to_vec(&activity.record)?;
        self.usernameid_record.insert(&key, record)?;

        let lap = rmps::to_vec(&activity.lap)?;
        self.usernameid_lap.insert(&key, lap)?;

        let gear_id = rmps::to_vec(&activity.gear_id)?;
        self.usernameid_gearid.insert(&key, gear_id)?;

        Ok(())
    }

    pub fn remove_activity(&self, query: &ActivityQuery) -> Result<()> {
        let key = query.to_key();

        self.usernameid_session.remove(&key)?;
        self.usernameid_record.remove(&key)?;
        self.usernameid_lap.remove(&key)?;
        self.usernameid_gearid.remove(&key)?;

        Ok(())
    }

    /*
    pub fn get_activity(&self, query: &ActivityQuery) -> Result<()> {
        let key = query.to_key();

        let session = self.get_session(&key)?;

        let record = self.get_record(&key)?;

        let lap = self.get_lap(&key)?;

        let gear_id = self.get_gear(&key)?;

        Ok(Activity {
            id: query.id.into(),
            gear_id,
            session,
            record,
            lap,
        })
    }
    */

    pub fn get_session(&self, query: &ActivityQuery) -> Result<Option<Session>> {
        Ok(self
            .usernameid_session
            .get(&query.to_key())?
            .as_deref()
            .map(rmps::from_read_ref)
            .transpose()?)
    }

    pub fn get_record(&self, query: &ActivityQuery) -> Result<Option<Record>> {
        Ok(self
            .usernameid_record
            .get(&query.to_key())?
            .as_deref()
            .map(rmps::from_read_ref)
            .transpose()?)
    }

    pub fn get_lap(&self, query: &ActivityQuery) -> Result<Option<Vec<Lap>>> {
        Ok(self
            .usernameid_lap
            .get(&query.to_key())?
            .as_deref()
            .map(rmps::from_read_ref)
            .transpose()?)
    }

    pub fn insert_gear(&self, query: &ActivityQuery, gear: &str) -> Result<Option<()>> {
        Ok(self
            .usernameid_gearid
            .insert(&query.to_key(), rmps::to_vec(gear)?)?
            .as_deref()
            .map(|_| ()))
    }

    pub fn get_gear(&self, query: &ActivityQuery) -> Result<Option<String>> {
        Ok(self
            .usernameid_gearid
            .get(&query.to_key())?
            .as_deref()
            .map(|x| rmps::from_read_ref(&x))
            .transpose()?)
    }

    pub fn username_iter_session(&self, user: &UserQuery) -> Result<impl Iterator<Item = Session>> {
        Ok(self
            .usernameid_session
            .scan_prefix(&user.to_prefix())
            .values()
            .rev()
            .flatten()
            .flat_map(|x| rmps::from_read_ref(&x)))
    }

    pub fn username_iter_record(&self, user: &UserQuery) -> Result<impl Iterator<Item = Record>> {
        Ok(self
            .usernameid_record
            .scan_prefix(&user.to_prefix())
            .values()
            .rev()
            .flatten()
            .flat_map(|x| rmps::from_read_ref(&x)))
    }

    pub fn username_iter_lap(&self, user: &UserQuery) -> Result<impl Iterator<Item = Vec<Lap>>> {
        Ok(self
            .usernameid_lap
            .scan_prefix(&user.to_prefix())
            .values()
            .rev()
            .flatten()
            .flat_map(|x| rmps::from_read_ref(&x)))
    }

    pub fn username_iter_gear(
        &self,
        user: &UserQuery,
    ) -> Result<impl Iterator<Item = Option<String>>> {
        Ok(self
            .usernameid_gearid
            .scan_prefix(&user.to_prefix())
            .values()
            .rev()
            .flatten()
            .flat_map(|x| rmps::from_read_ref(&x)))
    }

    pub fn prev(&self, query: &ActivityQuery) -> Result<Option<String>> {
        let user_query = UserQuery::from(query);

        let set: std::collections::BTreeSet<String> = self.username_iter_id(&user_query)?.collect();

        Ok(set.range(..query.id.to_string()).next_back().cloned())
    }

    pub fn next(&self, query: &ActivityQuery) -> Result<Option<String>> {
        let user_query = UserQuery::from(query);

        let set: std::collections::BTreeSet<String> = self.username_iter_id(&user_query)?.collect();

        Ok(set
            .range((
                std::ops::Bound::Excluded(query.id.to_string()),
                std::ops::Bound::Unbounded,
            ))
            .next()
            .cloned())
    }
}
