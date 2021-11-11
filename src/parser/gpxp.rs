use chrono::{offset::Local, DateTime};
use gpx::read;
use gpx::{Gpx, Track, TrackSegment};
use std::io::BufReader;

use gpx::errors::GpxError;

use geo::{Coordinate, Point};

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


use geo::prelude::*;

pub fn parse(gpx_data: &[u8], gear_id: Option<String>) -> Result<Activity> {
    let data = BufReader::new(gpx_data);
    let mut record = Record::default();
    let mut session = Session::default();

    let res: std::result::Result<Gpx, GpxError> = read(data);
    if let Err(_) = res {
        return Err(Error::ParseError);
    } else if let Ok(gpx) = res {

        if let Some(st) = gpx.tracks[0].segments[0].points[0].time {
            session.start_time = TimeStamp(DateTime::from(st));
        }

        // session.cadence_avg = Some(3u8);
        // session.cadence_max = Some(3u8);
        // session.heartrate_avg = Some(3u8);
        // session.heartrate_max = Some(3u8);
        // session.speed_avg = Some(Velocity::new::<meter_per_second>(3f64));
        // session.speed_max = Some(Velocity::new::<meter_per_second>(3f64));
        // session.power_avg = Some(3u16);
        // session.power_max = Some(3u16);
        // session.nec_lat = Some(3.0f64);
        // session.nec_lat = Some(3.0f64);
        // session.swc_lat = Some(3.0f64);
        // session.swc_lat = Some(3.0f64);
        session.laps = Some(0u16);
        session.activity_type = ActivityType::Running;
        // session.ascent = Some(Length_u16::new::<meter>(10u16));
        // session.descent = Some(Length_u16::new::<meter>(10u16));
        // session.calories = Some(3u16);
        // session.distance = Some(Length_f64::new::<meter>(10f64));
        // session.duration = Duration::from_secs_f64(10f64);
        // session.duration_active = Duration::from_secs_f64(10f64);

        let mut prev_coord : Option<Point<f64>> = None;
        let mut distance = Length_f64::new::<meter>(0f64);

        for track in gpx.tracks {
            for track_segment in track.segments {
                for wpt in track_segment.points {
                    let point = wpt.point();
                    distance = distance +
                        match prev_coord {
                            Some(p) => Length_f64::new::<meter>(p.geodesic_distance(&wpt.point())),
                            None => Length_f64::new::<meter>(0f64),
                        };

                    record.distance.push(Some(distance));

                    prev_coord = Some(wpt.point());
                    record.lat.push(Some(point.lat()));
                    record.lon.push(Some(point.lng()));

                    if let Some(ts) = wpt.time {
                        record.timestamp.push(TimeStamp(DateTime::from(ts)));
                    }

                     if let Some(e) = wpt.elevation {
                        record.altitude.push(Some(Length_f64::new::<meter>(e)));
                     }

                    if let Some(e) = wpt.speed {
                        record.speed.push(Some(Velocity::new::<meter_per_second>(e)));
                    }
                }
            }
        }
    }

//    let mut lap = Lap::default();
    let mut lap_vec = Vec::new();
//    lap_vec.push(lap);

    if session.nec_lat.is_none()
        || session.nec_lon.is_none()
        || session.swc_lat.is_none()
        || session.swc_lon.is_none()
    {
        session.nec_lat = Some(
            record
                .lat
                .iter()
                .flatten()
                .copied()
                .fold(f64::NAN, f64::max),
        );
        session.nec_lon = Some(
            record
                .lon
                .iter()
                .flatten()
                .copied()
                .fold(f64::NAN, f64::max),
        );
        session.swc_lat = Some(
            record
                .lat
                .iter()
                .flatten()
                .copied()
                .fold(f64::NAN, f64::min),
        );
        session.swc_lon = Some(
            record
                .lon
                .iter()
                .flatten()
                .copied()
                .fold(f64::NAN, f64::min),
        );
    }

    Ok(Activity {
        id: session.start_time.0.format("%Y%m%d%H%M").to_string(),
        gear_id,
        session,
        record,
        lap: lap_vec,
        notes: Some("une note".to_string()),
    })
}
