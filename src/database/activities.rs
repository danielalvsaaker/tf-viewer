use crate::{Error, Result, Activity};

#[derive(Clone)]
pub struct ActivityTree {
    pub(super) usernameid_activity: sled::Tree,
}

impl ActivityTree {
    pub fn exists(&self, id: String, username: String) -> Result<bool> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());
        Ok(self.usernameid_activity.contains_key(&key)?)
    }

    pub fn insert(&self, activity: Activity, username: String) -> Result<()> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(&[activity.id]);


        let serialized = bincode::serialize(&activity).expect("Failed to serialize activity");
        self.usernameid_activity.insert(key, serialized)?;
        Ok(())
    }

    pub fn iter(&self, username: String) -> sled::Iter {
        let mut prefix = username.as_bytes().to_vec();
        prefix.push(0xff);

        self.usernameid_activity.scan_prefix(&prefix)
    }

    pub fn get(&self, username: String, id: u8) -> Result<Activity> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(&[id]);

        let get = self.usernameid_activity.get(&key)?;
        Ok(bincode::deserialize::<Activity>(&get.unwrap()).unwrap())
    }
}
