use fitparser::profile::field_types::MesgNum;
use fitparser::FitDataField;
use fitparser::Value::*;
use std::{iter::FromIterator, str::FromStr, string::String, collections::HashMap};

use crate::{
    error::{Error, ErrorKind, Result},
    models::{Activity, ActivityType, Duration, Lap, Record, Session, TimeStamp},
};

macro_rules! map_value {
    ($name:ident, $type:ident, $( $pattern:pat )|+ => $mapping:expr) => {
        fn $name(v: &&fitparser::Value) -> Option<$type> {
            match v {
                $( $pattern )|+ => ::std::option::Option::Some($mapping),
                _ => ::std::option::Option::None,
            }
        }
    }
}

map_value!(map_uint8, u8, UInt8(x) => *x);
map_value!(map_uint16, u16, UInt16(x) => *x);
map_value!(map_sint32, i32, SInt32(x) => *x);
map_value!(map_float64, f64, Float64(x) => *x);
map_value!(map_string, String, String(x) => x.to_string());
map_value!(map_timestamp, TimeStamp, Timestamp(x) => TimeStamp(*x));

pub fn parse(fit_data: &[u8], gear_id: Option<String>) -> Result<Activity> {
    let mut session: Session = Session::default();
    let mut record: Record = Record::default();
    let mut lap_vec: Vec<Lap> = Vec::new();

    let file = fitparser::from_bytes(fit_data)
        .map_err(|_| Error::BadRequest(ErrorKind::BadRequest, "File is not a valid .fit-file"))?;

    if !file.iter().any(|x| x.kind() == MesgNum::Session) {
        return Err(Error::BadRequest(
            ErrorKind::BadRequest,
            "File does not contain session data",
        ));
    }

    for data in file {
        match data.kind() {
            MesgNum::Session => parse_session(data.fields(), &mut session)?,
            MesgNum::Record => parse_record(data.fields(), &mut record)?,
            MesgNum::Lap => {
                let mut lap = Lap::default();
                parse_lap(data.fields(), &mut lap)?;
                lap_vec.push(lap);
            }
            _ => (),
        }
    }

    // Some fit-files do not contain corner coordinates,
    // so find them manually if missing
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
                .fold(f64::NAN, |x, y| f64::max(x, *y)),
        );
        session.nec_lon = Some(
            record
                .lon
                .iter()
                .flatten()
                .fold(f64::NAN, |x, y| f64::max(x, *y)),
        );
        session.swc_lat = Some(
            record
                .lat
                .iter()
                .flatten()
                .fold(f64::NAN, |x, y| f64::min(x, *y)),
        );
        session.swc_lon = Some(
            record
                .lon
                .iter()
                .flatten()
                .fold(f64::NAN, |x, y| f64::min(x, *y)),
        );
    }


    Ok(Activity {
        id: session.start_time.0.format("%Y%m%d%H%M").to_string(),
        gear_id,
        session,
        record,
        lap: lap_vec,
    })
}

fn parse_session(fields: &[FitDataField], session: &mut Session) -> Result<()> {
    // Semicircle to degree
    let multiplier = 180f64 / 2f64.powi(31);

    let field_map: HashMap<&str, &fitparser::Value> =
        HashMap::from_iter(fields.iter().map(|x| (x.name(), x.value())));

    session.cadence_avg = field_map
        .get("avg_cadence")
        .and_then(map_uint8);

    session.cadence_max = field_map
        .get("max_cadence")
        .and_then(map_uint8);

    session.heartrate_avg = field_map
        .get("avg_heart_rate")
        .and_then(map_uint8);

    session.heartrate_max = field_map
        .get("max_heart_rate")
        .and_then(map_uint8);

    session.speed_avg = field_map
        .get("enhanced_avg_speed")
        .and_then(map_float64)
        .map(|x| (x * 3.6f64 * 100f64).round() / 100f64);

    session.speed_max = field_map
        .get("enhanced_max_speed")
        .and_then(map_float64)
        .map(|x| (x * 3.6f64 * 100f64).round() / 100f64);

    session.power_avg = field_map
        .get("avg_power")
        .and_then(map_uint16);

    session.power_max = field_map
        .get("max_power")
        .and_then(map_uint16);

    session.nec_lat = field_map
        .get("nec_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * multiplier);

    session.nec_lon = field_map
        .get("nec_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * multiplier);

    session.swc_lat = field_map
        .get("swc_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * multiplier);

    session.swc_lon = field_map
        .get("swc_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * multiplier);

    session.laps = field_map
        .get("num_laps")
        .and_then(map_uint16);

    session.activity_type = ActivityType::from_str(
        &field_map
            .get("sport")
            .and_then(map_string)
            .unwrap_or_default()
    ).unwrap_or_default();

    session.ascent = field_map
        .get("total_ascent")
        .and_then(map_uint16);

    session.descent = field_map
        .get("total_descent")
        .and_then(map_uint16);

    session.calories = field_map
        .get("total_calories")
        .and_then(map_uint16);

    session.distance = field_map
        .get("total_distance")
        .and_then(map_float64)
        .map(|x| (x / 10f64).round() / 100f64);

    session.duration = field_map
        .get("total_elapsed_time")
        .and_then(map_float64)
        .map(Duration::from_secs_f64)
        .unwrap_or_default();

    session.duration_active = field_map
        .get("total_timer_time")
        .and_then(map_float64)
        .map(Duration::from_secs_f64)
        .unwrap_or_default();

    session.start_time = field_map
        .get("start_time")
        .and_then(map_timestamp)
        .unwrap_or_default();


    Ok(())
}

fn parse_record(fields: &[FitDataField], record: &mut Record) -> Result<()> {
    // Semicircle to degree
    let multiplier = 180f64 / 2f64.powi(31);

    let field_map: HashMap<&str, &fitparser::Value> =
        HashMap::from_iter(fields.iter().map(|x| (x.name(), x.value())));

    record.cadence.push(
        field_map
            .get("cadence")
            .and_then(map_uint8)
    );

    record.distance.push(
        field_map
            .get("distance")
            .and_then(map_float64)
            .map(|x| (x / 10f64).round() / 100f64),
    );

    record.altitude.push(
        field_map
        .get("enhanced_altitude")
        .and_then(map_float64)
    );

    record.speed.push(
        field_map
            .get("enhanced_speed")
            .and_then(map_float64)
            .map(|x| (x * 3.6f64 * 100f64).round() / 100f64),
    );

    record.power.push(
        field_map
            .get("power")
            .and_then(map_uint16)
    );

    record.heartrate.push(
        field_map
            .get("heart_rate")
            .and_then(map_uint8)
    );

    record.lat.push(
        field_map
            .get("position_lat")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * multiplier),
    );

    record.lon.push(
        field_map
            .get("position_long")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * multiplier),
    );

    let timestamp = field_map
        .get("timestamp")
        .and_then(map_timestamp)
        .unwrap_or_default();

    let duration = match record.timestamp.first() {
        Some(x) => Duration::between(&timestamp, x),
        None => Duration::default(),
    };

    record.duration.push(duration);
    record.timestamp.push(timestamp);

    Ok(())
}

fn parse_lap(fields: &[FitDataField], lap: &mut Lap) -> Result<()> {
    // Semicircle to degree
    let multiplier = 180f64 / 2f64.powi(31);

    let field_map: HashMap<&str, &fitparser::Value> =
        HashMap::from_iter(fields.iter().map(|x| (x.name(), x.value())));

    lap.cadence_avg = field_map
        .get("avg_cadence")
        .and_then(map_uint8);

    lap.cadence_max = field_map
        .get("max_cadence")
        .and_then(map_uint8);

    lap.heartrate_avg = field_map
        .get("avg_heart_rate")
        .and_then(map_uint8);

    lap.heartrate_max = field_map
        .get("max_heart_rate")
        .and_then(map_uint8);

    lap.speed_avg = field_map
        .get("enhanced_avg_speed")
        .and_then(map_float64)
        .map(|x| (x * 3.6f64 * 100f64).round() / 100f64);

    lap.speed_max = field_map
        .get("enhanced_max_speed")
        .and_then(map_float64)
        .map(|x| (x * 3.6f64 * 100f64).round() / 100f64);

    lap.power_avg = field_map
        .get("avg_power")
        .and_then(map_uint16);

    lap.power_max = field_map
        .get("max_power")
        .and_then(map_uint16);

    lap.lat_start = field_map
        .get("start_position_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * multiplier);

    lap.lon_start = field_map
        .get("start_position_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * multiplier);

    lap.lat_end = field_map
        .get("end_position_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * multiplier);

    lap.lon_end = field_map
        .get("end_position_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * multiplier);

    lap.ascent = field_map
        .get("total_ascent")
        .and_then(map_uint16);

    lap.descent = field_map
        .get("total_descent")
        .and_then(map_uint16);

    lap.calories = field_map
        .get("total_calories")
        .and_then(map_uint16);

    lap.distance = field_map
        .get("total_distance")
        .and_then(map_float64)
        .map(|x| (x / 10f64).round() / 100f64);

    lap.duration = field_map
        .get("total_elapsed_time")
        .and_then(map_float64)
        .map(Duration::from_secs_f64)
        .unwrap_or_default();

    lap.duration_active = field_map
        .get("total_timer_time")
        .and_then(map_float64)
        .map(Duration::from_secs_f64)
        .unwrap_or_default();

    Ok(())
}
