use crate::{
    error::{Error, ErrorKind, Result},
    models::{Activity, Duration, Lap, Record, Session, UserTotals},
};
use chrono::{self, Datelike, Local};
use rmp_serde as rmps;

use uom::si::f64::Length;
use uom::si::length::meter;

#[derive(Clone)]
pub struct ActivityTree {
    pub(super) usernameid_gearid: sled::Tree,
    pub(super) usernameid_session: sled::Tree,
    pub(super) usernameid_record: sled::Tree,
    pub(super) usernameid_lap: sled::Tree,
    pub(super) usernameid_notes: sled::Tree,
}

impl ActivityTree {
    pub fn exists(&self, username: &str, id: &str) -> Result<bool> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());
        Ok(self.usernameid_session.contains_key(&key)?)
    }

    pub fn insert(&self, activity: Activity, username: &str) -> Result<()> {
        if self.exists(username, &activity.id)? {
            let existing = self.get_activity(username, &activity.id)?;
            let activity = Activity {
                id: existing.id,
                gear_id: existing.gear_id,
                notes: existing.notes,
                ..activity
            };

            self.insert_or_overwrite(activity, username)
        } else {
            self.insert_or_overwrite(activity, username)
        }
    }

    pub fn insert_or_overwrite(&self, activity: Activity, username: &str) -> Result<()> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(&activity.id.as_bytes());

        let session = rmps::to_vec(&activity.session)?;
        self.usernameid_session.insert(&key, session)?;

        let record = rmps::to_vec(&activity.record)?;
        self.usernameid_record.insert(&key, record)?;

        let lap = rmps::to_vec(&activity.lap)?;
        self.usernameid_lap.insert(&key, lap)?;

        let gear_id = rmps::to_vec(&activity.gear_id)?;
        self.usernameid_gearid.insert(&key, gear_id)?;

        if let Some(mut x) = activity.notes {
            x.truncate(300);
            self.usernameid_notes.insert(&key, x.as_bytes())?;
        } else {
            self.usernameid_notes.remove(&key)?;
        }

        Ok(())
    }

    pub fn user_totals(&self, username: &str) -> Result<UserTotals> {
        let iter = self
            .username_iter_session(username)?
            .collect::<Vec<Session>>();
        let cycling_iter = iter.iter().filter(|x| x.activity_type.is_cycling());

        let running_iter = iter.iter().filter(|x| x.activity_type.is_running());

        let fold = |acc: (Length, Duration, usize), x: &Session| {
            (
                acc.0 + x.distance.unwrap_or_default(),
                acc.1 + x.duration_active,
                acc.2 + 1_usize,
            )
        };

        let cycling_month = cycling_iter
            .clone()
            .filter(|x| x.start_time.0 > (Local::now() - chrono::Duration::days(30)))
            .fold((Length::new::<meter>(0.), Duration::default(), 0), fold);

        let running_month = running_iter
            .clone()
            .filter(|x| x.start_time.0 > (Local::now() - chrono::Duration::days(30)))
            .fold((Length::new::<meter>(0.), Duration::default(), 0), fold);

        let cycling_year = cycling_iter
            .clone()
            .filter(|x| x.start_time.0.year() == Local::now().year())
            .fold((Length::new::<meter>(0.), Duration::default(), 0), fold);

        let running_year = running_iter
            .clone()
            .filter(|x| x.start_time.0.year() == Local::now().year())
            .fold((Length::new::<meter>(0.), Duration::default(), 0), fold);

        let cycling_all =
            cycling_iter.fold((Length::new::<meter>(0.), Duration::default(), 0), fold);

        let running_all =
            running_iter.fold((Length::new::<meter>(0.), Duration::default(), 0), fold);

        Ok(UserTotals {
            cycling_month,
            cycling_year,
            cycling_all,
            running_month,
            running_year,
            running_all,
        })
    }

    pub fn gear_totals(&self, username: &str, gear: &str) -> Result<(Length, Duration)> {
        Ok(self
            .username_iter_session(username)?
            .zip(self.username_iter_gear(username)?)
            .filter(|(_, y)| y.as_deref() == Some(gear))
            .map(|(x, _)| x)
            .fold((Length::new::<meter>(0.), Duration::default()), |acc, x| {
                (
                    acc.0 + x.distance.unwrap_or_default(),
                    acc.1 + x.duration_active,
                )
            }))
    }

    pub fn username_iter_gear(
        &self,
        username: &str,
    ) -> Result<impl Iterator<Item = Option<String>>> {
        let mut prefix = username.as_bytes().to_vec();
        prefix.push(0xff);

        Ok(self
            .usernameid_gearid
            .scan_prefix(&prefix)
            .values()
            .rev()
            .flatten()
            .flat_map(|x| rmps::from_read_ref(&x)))
    }

    pub fn username_iter_session(&self, username: &str) -> Result<impl Iterator<Item = Session>> {
        let mut prefix = username.as_bytes().to_vec();
        prefix.push(0xff);

        Ok(self
            .usernameid_session
            .scan_prefix(&prefix)
            .values()
            .rev()
            .flatten()
            .flat_map(|x| rmps::from_read_ref(&x)))
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

    pub fn get_session(&self, username: &str, id: &str) -> Result<Session> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());

        self.usernameid_session
            .get(&key)?
            .and_then(|x| rmps::from_read_ref(&x).ok())
            .ok_or(Error::BadRequest(ErrorKind::NotFound, "Session not found"))
    }

    pub fn get_record(&self, username: &str, id: &str) -> Result<Record> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());

        self.usernameid_record
            .get(&key)?
            .and_then(|x| rmps::from_read_ref(&x).ok())
            .ok_or(Error::BadRequest(ErrorKind::NotFound, "Record not found"))
    }

    pub fn get_lap(&self, username: &str, id: &str) -> Result<Vec<Lap>> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());

        self.usernameid_lap
            .get(&key)?
            .and_then(|x| rmps::from_read_ref(&x).ok())
            .ok_or(Error::BadRequest(ErrorKind::NotFound, "Laps not found"))
    }

    pub fn get_gear_id(&self, username: &str, id: &str) -> Result<Option<String>> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());

        self.usernameid_gearid
            .get(&key)?
            .and_then(|x| rmps::from_read_ref(&x).ok())
            .ok_or(Error::BadRequest(ErrorKind::NotFound, "Gear not found"))
    }

    pub fn get_notes(&self, username: &str, id: &str) -> Result<Option<String>> {
        let mut key = username.as_bytes().to_vec();
        key.push(0xff);
        key.extend_from_slice(id.as_bytes());

        Ok(self
            .usernameid_notes
            .get(&key)?
            .map(|x| x.to_vec())
            .and_then(|x| String::from_utf8(x).ok()))
    }

    pub fn get_activity(&self, username: &str, id: &str) -> Result<Activity> {
        Ok(Activity {
            id: id.to_owned(),
            gear_id: self.get_gear_id(username, id)?,
            session: self.get_session(username, id)?,
            record: self.get_record(username, id)?,
            lap: self.get_lap(username, id)?,
            notes: self.get_notes(username, id)?,
        })
    }
}
