//use chrono::{offset::Local, DateTime, Utc};
use chrono::prelude::*;

use gpx::{errors::GpxError, read, Gpx, Waypoint};

use std::io::BufReader;

use geo::Point;

use uom::si::{
    f64::*,
    f64::{Length as Length_f64, Velocity},
    length::meter,
    time::second,
    velocity::meter_per_second,
    Quantity,
};

use crate::{
    error::{Error, Result},
    models::{Activity, ActivityType, Duration, Record, Session, TimeStamp},
};

use geo::prelude::*;

pub fn parse(gpx_data: &[u8], gear_id: Option<String>) -> Result<Activity> {
    let data = BufReader::new(gpx_data);
    let mut record = Record::default();
    let mut session = Session::default();

    let res: std::result::Result<Gpx, GpxError> = read(data);
    if res.is_err() {
        return Err(Error::ParseError);
    } else if let Ok(gpx) = res {
        if let Some(st) = gpx.tracks[0].segments[0].points[0].time {
            session.start_time = TimeStamp(DateTime::from(st));
        }
        session.laps = Some(0u16);
        session.activity_type = ActivityType::Running;

        let mut prev_coord: Option<Point<f64>> = None;
        let mut prev_timestamp: Option<DateTime<Utc>> = None;

        let mut session_avg_speed = Velocity::new::<meter_per_second>(f64::NAN);
        let mut session_max_speed = Velocity::new::<meter_per_second>(f64::NAN);

        let mut session_nec_lat = f64::NAN;
        let mut session_nec_lon = f64::NAN;
        let mut session_swc_lat = f64::NAN;
        let mut session_swc_lon = f64::NAN;

        let mut distance = Length_f64::new::<meter>(0f64);
        let mut first_wpt: Option<Waypoint> = None;
        let mut last_wpt: Option<Waypoint> = None;

        for track in gpx.tracks {
            for track_segment in track.segments {
                for wpt in track_segment.points {
                    let point = wpt.point();

                    let delta_distance = match prev_coord {
                        Some(p) => Length_f64::new::<meter>(p.geodesic_distance(&wpt.point())),
                        None => Length_f64::new::<meter>(0f64),
                    };

                    distance += delta_distance;

                    record.distance.push(Some(distance));

                    record.lat.push(Some(point.lat()));
                    record.lon.push(Some(point.lng()));

                    session_nec_lat = f64::max(session_nec_lat, point.lat());
                    session_nec_lon = f64::max(session_nec_lon, point.lng());
                    session_swc_lat = f64::min(session_swc_lat, point.lat());
                    session_swc_lon = f64::min(session_swc_lon, point.lng());

                    let mut cur_timestamp: Option<DateTime<Utc>> = None;
                    if let Some(ts) = wpt.time {
                        //                        cur_timestamp = Some(DateTime::from(ts));
                        cur_timestamp = Some(ts);
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
                        record
                            .speed
                            .push(Some(Velocity::new::<meter_per_second>(e)));
                    } else if let Some(prev_ts) = prev_timestamp {
                        let cur_ts = cur_timestamp.unwrap();
                        let duration = cur_ts - prev_ts;
                        let speed =
                            delta_distance / Time::new::<second>(duration.num_seconds() as f64);
                        record.speed.push(Some(speed));
                        session_max_speed = Quantity::max(session_max_speed, speed);
                    }
                    prev_coord = Some(wpt.point());
                    if cur_timestamp != None {
                        prev_timestamp = cur_timestamp;
                    }

                    if first_wpt == None {
                        first_wpt = Some(wpt);
                    } else {
                        last_wpt = Some(wpt);
                    }
                }
            }
        }

        session.nec_lat = Some(session_nec_lat);
        session.nec_lon = Some(session_nec_lon);
        session.swc_lat = Some(session_swc_lat);
        session.swc_lon = Some(session_swc_lon);

        if first_wpt != None && last_wpt != None {
            if let (Some(first_wpt), Some(last_wpt)) = (first_wpt, last_wpt) {
                if let (Some(fts), Some(lts)) = (first_wpt.time, last_wpt.time) {
                    let full_duration = lts - fts;
                    session_avg_speed =
                        distance / Time::new::<second>(full_duration.num_seconds() as f64);
                    session.duration =
                        Duration::from_secs_f64(full_duration.num_seconds() as f64);
                    session.duration_active =
                        Duration::from_secs_f64(full_duration.num_seconds() as f64);
                }
            }
        }
        //        let full_duration =
        //        session.speed_avg = Some(distance / );
        session.speed_avg = Some(session_avg_speed);
        session.speed_max = Some(session_max_speed);
    }

    let lap_vec = Vec::new();

    Ok(Activity {
        id: session.start_time.0.format("%Y%m%d%H%M").to_string(),
        gear_id,
        session,
        record,
        lap: lap_vec,
        notes: Some("une note".to_string()),
    })
}
