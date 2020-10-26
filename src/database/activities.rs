use crate::{Activity, Session, Record, Lap};
use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct ActivityTree {
    pub usernameid_id: sled::Tree,
    pub(super) usernameid_gear_id: sled::Tree,
    pub(super) usernameid_session: sled::Tree,
    pub(super) usernameid_record: sled::Tree,
    pub(super) usernameid_lap: sled::Tree,
}

impl ActivityTree {
    pub fn exists(&self, username: &str, id: &str) -> Result<bool> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());
        Ok(self.usernameid_id.contains_key(&key)?)
    }

    pub fn insert(&self, activity: Activity, username: &str) -> Result<()> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(activity.id.as_bytes());

        let session = bincode::serialize(&activity.session)?;
        self.usernameid_session.insert(&key, session)?;

        let record = bincode::serialize(&activity.record)?;
        self.usernameid_record.insert(&key, record)?;

        let lap = bincode::serialize(&activity.lap)?;
        self.usernameid_lap.insert(&key, lap)?;

        self.usernameid_id.insert(&key, activity.id.as_bytes());
        self.usernameid_gear_id.insert(&key, activity.gear_id.as_bytes());

        Ok(())
    }

    pub fn iter(&self, username: &str) -> Result<impl Iterator<Item = Session>> {
        let mut prefix = username.as_bytes().to_vec();
        prefix.push(0xff);

        Ok(self.usernameid_session.scan_prefix(&prefix)
            .values()
            .rev()
            .flatten()
            .map(|x| bincode::deserialize::<Session>(&x).unwrap())
        )
        
    }

    pub fn iter_all_id(&self, amount: usize) -> Result<impl Iterator<Item = String>> {
        Ok(self.usernameid_id.iter()
           .values()
           .rev()
           .take(amount)
           .map(|x| String::from_utf8(x.unwrap().to_vec()).unwrap())
        )
    }

    pub fn iter_session(&self, amount: usize) -> Result<impl Iterator<Item = Session>> {
        Ok(self.usernameid_session.iter()
            .values()
            .rev()
            .take(amount)
            .map(|x| bincode::deserialize::<Session>(&x.unwrap()).unwrap())
        )
    }

    pub fn iter_record(&self, amount: usize) -> Result<impl Iterator<Item = Record>> {
        Ok(self.usernameid_record.iter()
            .values()
            .rev()
            .take(amount)
            .map(|x| bincode::deserialize::<Record>(&x.unwrap()).unwrap())
        )
    }

    pub fn iter_id(&self, username: &str) -> Result<impl Iterator<Item = String>> {
        let mut prefix = username.as_bytes().to_vec();
        prefix.push(0xff);

        Ok(self.usernameid_id.scan_prefix(&prefix)
           .values()
           .rev()
           .map(|x| String::from_utf8(x.unwrap().to_vec()).unwrap())
           )
    }

    pub fn get_session(&self, username: &str, id: &str) -> Result<Session> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());

        let get = self.usernameid_session.get(&key)?;

        match get {
            Some(x) => Ok(bincode::deserialize::<Session>(&x)?),
            None => Err(anyhow!("Failed to deserialize session")),
        }
    }

    pub fn get_record(&self, username: &str, id: &str) -> Result<Record> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());

        let get = self.usernameid_record.get(&key)?;

        match get {
            Some(x) => Ok(bincode::deserialize::<Record>(&x)?),
            None => Err(anyhow!("Failed to deserialize record")),
        }
    }

    pub fn get_lap(&self, username: &str, id: &str) -> Result<Vec<Lap>> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());

        let get = self.usernameid_lap.get(&key)?;

        match get {
            Some(x) => Ok(bincode::deserialize::<Vec<Lap>>(&x)?),
            None => Err(anyhow!("Failed to deserialize laps")),
        }
    }

    pub fn get_gear_id(&self, username: &str, id: &str) -> Result<String> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());

        let get = self.usernameid_gear_id.get(&key)?;

        match get {
            Some(x) => Ok(String::from_utf8(x.to_vec())?),
            None => Err(anyhow!("Failed to get gear id")),
        }
    }

    pub fn get_activity(&self, username: &str, id: &str) -> Result<Activity> {
        Ok(Activity {
            id: id.to_owned(),
            gear_id: self.get_gear_id(username, id)?,
            session: self.get_session(username, id)?,
            record: self.get_record(username, id)?,
            lap: self.get_lap(username, id)?,
        })
    }
}
