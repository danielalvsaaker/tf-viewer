use chrono::{offset::Local, DateTime};
use gpx::read;
use gpx::{Gpx, Track, TrackSegment};
use std::io::BufReader;

use gpx::errors::GpxError;

use uom::si::{
    f64::{Length as Length_f64, Velocity},
    length::meter,
    u16::Length as Length_u16,
    velocity::meter_per_second,
};


use crate::{
    error::{Result, Error},
    models::{Activity, ActivityType, Duration, Lap, Record, Session, TimeStamp},
};


pub fn parse(gpx_data: &[u8], gear_id: Option<String>) -> Result<Activity> {
    let data = BufReader::new(gpx_data);
    let mut record = Record::default();
    let session = Session::default();

    let res: std::result::Result<Gpx, GpxError> = read(data);
    if let Err(_) = res {
        return Err(Error::ParseError);
    } else if let Ok(gpx) = res {
        for track in gpx.tracks {
            for track_segment in track.segments {
                for wpt in track_segment.points {
                    let point = wpt.point();
                    record.lat.push(Some(point.lat()));
                    record.lon.push(Some(point.lng()));

                    if let Some(ts) = wpt.time {
                        record.timestamp.push(TimeStamp(DateTime::from(ts)));
                    }

                     if let Some(e) = wpt.elevation {
                        record.altitude.push(Some(Length_f64::new::<meter>(e)));
                    }
                }
            }
        }
    }



    Ok(Activity {
        id: "202111100744".to_string(),
        gear_id,
        session,
        record,
        lap: Vec::new(),
        notes: None,
    })
}
