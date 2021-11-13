//use chrono::{offset::Local, DateTime, Utc};
use chrono::prelude::*;

use gpx::read;
use gpx::{Gpx, Track, TrackSegment};
use std::io::BufReader;

use gpx::errors::GpxError;

use geo::{Coordinate, Point};

use uom::si::f64::*;
use uom::si::length::kilometer;
use uom::si::time::second;

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

        // session.speed_avg = Some();
        // session.speed_max = Some(Velocity::new::<meter_per_second>(3f64));

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
        let mut prev_timestamp : Option<DateTime<Utc>> = None;

        let mut session_avg_speed = Velocity::new::<meter_per_second>(0f64);
        let mut session_max_speed = Velocity::new::<meter_per_second>(f64::NAN);

        let mut session_nec_lat = f64::NAN;
        let mut session_nec_lon = f64::NAN;
        let mut session_swc_lat = f64::NAN;
        let mut session_swc_lon = f64::NAN;

        let mut distance = Length_f64::new::<meter>(0f64);

        for track in gpx.tracks {
            for track_segment in track.segments {
                for wpt in track_segment.points {
                    let point = wpt.point();
                    let delta_distance = match prev_coord {
                        Some(p) => Length_f64::new::<meter>(p.geodesic_distance(&wpt.point())),
                        None => Length_f64::new::<meter>(0f64),
                    };

                    distance = distance + delta_distance;

                    record.distance.push(Some(distance));

                    record.lat.push(Some(point.lat()));
                    record.lon.push(Some(point.lng()));

                    session_nec_lat = f64::max(session_nec_lat, point.lat());
                    session_nec_lon = f64::max(session_nec_lon, point.lng());
                    session_swc_lat = f64::min(session_swc_lat, point.lat());
                    session_swc_lon = f64::min(session_swc_lon, point.lng());

                    let mut cur_timestamp : Option<DateTime<Utc>> = None;
                    if let Some(ts) = wpt.time {
                        cur_timestamp = Some(DateTime::from(ts));
                        record.timestamp.push(TimeStamp(DateTime::from(ts)));
                    }

                    if let Some(e) = wpt.elevation {
                        record.altitude.push(Some(Length_f64::new::<meter>(e)));
                    }

                    // GPX 1.0 accepts a speed tag for waypoint, which gpx crate expose in wpt.speed.
                    //
                    // GPX 1.1 does not have any speed info. These can be part
                    // of an extension, which gpx crate does not support. If
                    // there is no speed info, try to compute it from the
                    // distance and timestamps, if available.

                    if let Some(e) = wpt.speed {
                        record.speed.push(Some(Velocity::new::<meter_per_second>(e)));
                    } else {
                        // Compute speed
                        if let Some(prev_ts) = prev_timestamp {
                            let cur_ts = cur_timestamp.unwrap();
                            let duration = cur_ts - prev_ts;
                            let speed = delta_distance / Time::new::<second>(duration.num_seconds() as f64);
                            record.speed.push(Some(speed));
                        }
                    }
                    prev_coord = Some(wpt.point());
                    if cur_timestamp != None {
                        prev_timestamp = cur_timestamp;
                    }
                }
            }
        }
        session.nec_lat = Some(session_nec_lat);
        session.nec_lon = Some(session_nec_lon);
        session.swc_lat = Some(session_swc_lat);
        session.swc_lon = Some(session_swc_lon);
        session.speed_avg = Some(session_max_speed);
        session.speed_max = Some(session_avg_speed);
    }

    //    let mut lap = Lap::default();
    let mut lap_vec = Vec::new();
    //    lap_vec.push(lap);

    // if session.nec_lat.is_none()
    //     || session.nec_lon.is_none()
    //     || session.swc_lat.is_none()
    //     || session.swc_lon.is_none()
    // {
    //     session.nec_lat = Some(
    //         record
    //             .lat
    //             .iter()
    //             .flatten()
    //             .copied()
    //             .fold(f64::NAN, f64::max),
    //     );
    //     session.nec_lon = Some(
    //         record
    //             .lon
    //             .iter()
    //             .flatten()
    //             .copied()
    //             .fold(f64::NAN, f64::max),
    //     );
    //     session.swc_lat = Some(
    //         record
    //             .lat
    //             .iter()
    //             .flatten()
    //             .copied()
    //             .fold(f64::NAN, f64::min),
    //     );
    //     session.swc_lon = Some(
    //         record
    //             .lon
    //             .iter()
    //             .flatten()
    //             .copied()
    //             .fold(f64::NAN, f64::min),
    //     );
    // }

    Ok(Activity {
        id: session.start_time.0.format("%Y%m%d%H%M").to_string(),
        gear_id,
        session,
        record,
        lap: lap_vec,
        notes: Some("une note".to_string()),
    })
}
