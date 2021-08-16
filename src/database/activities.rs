use crate::{
    error::{Error, Result},
    models::*
};
use chrono::{self, Datelike, Local};
use rmp_serde as rmps;
use uom::si::{f64::Length, length::meter};
use actix_web::http::StatusCode;

#[derive(Clone)]
pub struct ActivityTree {
    pub(super) usernameid_gearid: sled::Tree,
    pub(super) usernameid_session: sled::Tree,
    pub(super) usernameid_record: sled::Tree,
    pub(super) usernameid_lap: sled::Tree,
    pub(super) usernameid_notes: sled::Tree,
}

impl ActivityTree {
    pub fn exists<Q: Query>(&self, query: &Q) -> Result<bool> {
        Ok(self.usernameid_session.contains_key(&query.to_key())?)
    }

    pub fn insert(&self, activity: Activity) -> Result<()> {
        let query: QueryKeyRef = (&activity).into();

        if self.exists(&query)? {
            let existing = self.get_activity(&query)?;
            let activity = Activity {
                id: existing.id,
                gear_id: existing.gear_id,
                notes: existing.notes,
                session: Session {
                    activity_type: existing.session.activity_type,
                    ..activity.session
                },
                ..activity
            };

            self.insert_or_overwrite(activity)
        } else {
            self.insert_or_overwrite(activity)
        }
    }

    pub fn insert_or_overwrite(&self, activity: Activity) -> Result<()> {
        let query: QueryKeyRef = (&activity).into();
        let key = query.to_key();

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

    pub fn user_totals(&self, user: &UserQuery) -> Result<UserTotals> {
        let iter = self
            .username_iter_session(user)?
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

    pub fn gear_totals<Q: Query>(&self, query: Q) -> Result<(Length, Duration)> {
        let user: UserQueryKeyRef = (&query).into();

        Ok(self
            .username_iter_session(&user)?
            .zip(self.username_iter_gear(&user)?)
            .filter(|(_, y)| y.as_deref() == Some(query.id()))
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

    pub fn username_iter_session(&self, user: &UserQuery) -> Result<impl Iterator<Item = Session>> {
        Ok(self
            .usernameid_session
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

    pub fn get_session<Q: Query>(&self, query: &Q) -> Result<Session> {
        self.usernameid_session
            .get(&query.to_key())?
            .and_then(|x| rmps::from_read_ref(&x).ok())
            .ok_or(Error::BadRequest(StatusCode::NOT_FOUND, "Session not found"))
    }

    pub fn get_record<Q: Query>(&self, query: &Q) -> Result<Record> {
        self.usernameid_record
            .get(&query.to_key())?
            .and_then(|x| rmps::from_read_ref(&x).ok())
            .ok_or(Error::BadRequest(StatusCode::NOT_FOUND, "Record not found"))
    }

    pub fn get_lap<Q: Query>(&self, query: &Q) -> Result<Vec<Lap>> {
        self.usernameid_lap
            .get(&query.to_key())?
            .and_then(|x| rmps::from_read_ref(&x).ok())
            .ok_or(Error::BadRequest(StatusCode::NOT_FOUND, "Laps not found"))
    }

    pub fn get_gear_id<Q: Query>(&self, query: &Q) -> Result<Option<String>> {
        self.usernameid_gearid
            .get(&query.to_key())?
            .and_then(|x| rmps::from_read_ref(&x).ok())
            .ok_or(Error::BadRequest(StatusCode::NOT_FOUND, "Gear not found"))
    }

    pub fn get_notes<Q: Query>(&self, query: &Q) -> Result<Option<String>> {
        Ok(self
            .usernameid_notes
            .get(&query.to_key())?
            .map(|x| x.to_vec())
            .and_then(|x| String::from_utf8(x).ok()))
    }

    pub fn get_activity<Q: Query>(&self, query: &Q) -> Result<Activity> {
        Ok(Activity {
            username: query.username().to_string(),
            id: query.id().to_string(),
            gear_id: self.get_gear_id(query)?,
            session: self.get_session(query)?,
            record: self.get_record(query)?,
            lap: self.get_lap(query)?,
            notes: self.get_notes(query)?,
        })
    }
}
