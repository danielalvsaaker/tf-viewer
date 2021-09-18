use fitparser::{de::DecodeOption, profile::field_types::MesgNum, FitDataField, Value};
use std::time::Duration;
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    str::FromStr,
};

use crate::error::{Error, Result};

use chrono::{offset::Local, DateTime};
use tf_models::{
    backend::{Lap, Record, Session},
    Activity, ActivityType,
};

use uom::si::{
    f64::{Length as Length_f64, Velocity},
    length::meter,
    power::watt,
    u16::Power,
    u32::Length as Length_u32,
    velocity::meter_per_second,
};

macro_rules! map_value {
    ($name:ident, $type:path, $( $pattern:pat )|+ => $mapping:expr) => {
        fn $name(v: &&fitparser::Value) -> Option<$type> {
            match v {
                $( $pattern )|+ => ::std::option::Option::Some($mapping),
                _ => ::std::option::Option::None,
            }
        }
    }
}

map_value!(map_uint8, u8, Value::UInt8(x) => *x);
map_value!(map_uint16, u16, Value::UInt16(x) => *x);
map_value!(map_sint32, i32, Value::SInt32(x) => *x);
map_value!(map_float64, f64, Value::Float64(x) => *x);
map_value!(map_string, String, Value::String(x) => x.to_string());
map_value!(map_timestamp, DateTime<Local>, Value::Timestamp(x) => *x);

fn between(lhs: &Option<DateTime<Local>>, rhs: Option<DateTime<Local>>) -> Option<Duration> {
    if let Some((x, y)) = lhs.zip(rhs) {
        chrono::Duration::to_std(&x.signed_duration_since(y)).ok()
    } else {
        None
    }
}

const MULTIPLIER: f64 = 180_f64 / (2_u32 << 30) as f64;

pub fn parse(fit_data: &[u8], gear_id: Option<String>) -> Result<Activity> {
    let mut session: Session = Session::default();
    let mut record: Record = Record::default();
    let mut lap_vec: Vec<Lap> = Vec::new();

    let options = HashSet::from_iter([DecodeOption::SkipHeaderCrcValidation]);
    let file = fitparser::de::from_bytes_with_options(fit_data, &options)?;

    if !file.iter().any(|x| x.kind() == MesgNum::Session) {
        return Err(Error::MissingData);
    }

    for data in file {
        match data.kind() {
            MesgNum::Session => parse_session(data.fields(), &mut session),
            MesgNum::Record => parse_record(data.fields(), &mut record),
            MesgNum::Lap => {
                let mut lap = Lap::default();
                parse_lap(data.fields(), &mut lap);
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
        id: session
            .start_time
            .ok_or(Error::MissingData)?
            .format("%Y%m%d%H%M")
            .to_string(),
        gear_id,
        session,
        record,
        lap: lap_vec,
    })
}

fn parse_session(fields: &[FitDataField], session: &mut Session) {
    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();

    session.cadence_avg = field_map.get("avg_cadence").and_then(map_uint8);

    session.cadence_max = field_map.get("max_cadence").and_then(map_uint8);

    session.heartrate_avg = field_map.get("avg_heart_rate").and_then(map_uint8);

    session.heartrate_max = field_map.get("max_heart_rate").and_then(map_uint8);

    session.speed_avg = field_map
        .get("enhanced_avg_speed")
        .and_then(map_float64)
        .map(Velocity::new::<meter_per_second>);

    session.speed_max = field_map
        .get("enhanced_max_speed")
        .and_then(map_float64)
        .map(Velocity::new::<meter_per_second>);

    session.power_avg = field_map
        .get("avg_power")
        .and_then(map_uint16)
        .map(Power::new::<watt>);

    session.power_max = field_map
        .get("max_power")
        .and_then(map_uint16)
        .map(Power::new::<watt>);

    session.nec_lat = field_map
        .get("nec_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    session.nec_lon = field_map
        .get("nec_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    session.swc_lat = field_map
        .get("swc_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    session.swc_lon = field_map
        .get("swc_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    session.laps = field_map.get("num_laps").and_then(map_uint16);

    session.activity_type = ActivityType::from_str(
        &field_map
            .get("sport")
            .and_then(map_string)
            .unwrap_or_default(),
    )
    .unwrap_or_default();

    session.ascent = field_map
        .get("total_ascent")
        .and_then(map_uint16)
        .map(u32::from)
        .map(Length_u32::new::<meter>);

    session.descent = field_map
        .get("total_descent")
        .and_then(map_uint16)
        .map(u32::from)
        .map(Length_u32::new::<meter>);

    session.calories = field_map.get("total_calories").and_then(map_uint16);

    session.distance = field_map
        .get("total_distance")
        .and_then(map_float64)
        .map(Length_f64::new::<meter>);

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

    session.start_time = field_map.get("start_time").and_then(map_timestamp);
}

fn parse_record(fields: &[FitDataField], record: &mut Record) {
    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();

    record
        .cadence
        .push(field_map.get("cadence").and_then(map_uint8));

    record.distance.push(
        field_map
            .get("distance")
            .and_then(map_float64)
            .map(Length_f64::new::<meter>),
    );

    record.altitude.push(
        field_map
            .get("enhanced_altitude")
            .and_then(map_float64)
            .map(Length_f64::new::<meter>),
    );

    record.speed.push(
        field_map
            .get("enhanced_speed")
            .and_then(map_float64)
            .map(Velocity::new::<meter_per_second>),
    );

    record.power.push(
        field_map
            .get("power")
            .and_then(map_uint16)
            .map(Power::new::<watt>),
    );

    record
        .heartrate
        .push(field_map.get("heart_rate").and_then(map_uint8));

    record.lat.push(
        field_map
            .get("position_lat")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * MULTIPLIER),
    );

    record.lon.push(
        field_map
            .get("position_long")
            .and_then(map_sint32)
            .map(|x| f64::from(x) * MULTIPLIER),
    );

    let timestamp = field_map.get("timestamp").and_then(map_timestamp);

    let duration = record
        .timestamp
        .first()
        .and_then(|x| between(&timestamp, *x))
        .unwrap_or_default();

    record.duration.push(duration);
    record.timestamp.push(timestamp);
}

fn parse_lap(fields: &[FitDataField], lap: &mut Lap) {
    let field_map: HashMap<&str, &fitparser::Value> =
        fields.iter().map(|x| (x.name(), x.value())).collect();

    lap.cadence_avg = field_map.get("avg_cadence").and_then(map_uint8);

    lap.cadence_max = field_map.get("max_cadence").and_then(map_uint8);

    lap.heartrate_avg = field_map.get("avg_heart_rate").and_then(map_uint8);

    lap.heartrate_max = field_map.get("max_heart_rate").and_then(map_uint8);

    lap.speed_avg = field_map
        .get("enhanced_avg_speed")
        .and_then(map_float64)
        .map(Velocity::new::<meter_per_second>);

    lap.speed_max = field_map
        .get("enhanced_max_speed")
        .and_then(map_float64)
        .map(Velocity::new::<meter_per_second>);

    lap.power_avg = field_map
        .get("avg_power")
        .and_then(map_uint16)
        .map(Power::new::<watt>);

    lap.power_max = field_map
        .get("max_power")
        .and_then(map_uint16)
        .map(Power::new::<watt>);

    lap.lat_start = field_map
        .get("start_position_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    lap.lon_start = field_map
        .get("start_position_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    lap.lat_end = field_map
        .get("end_position_lat")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    lap.lon_end = field_map
        .get("end_position_long")
        .and_then(map_sint32)
        .map(|x| f64::from(x) * MULTIPLIER);

    lap.ascent = field_map
        .get("total_ascent")
        .and_then(map_uint16)
        .map(u32::from)
        .map(Length_u32::new::<meter>);

    lap.descent = field_map
        .get("total_descent")
        .and_then(map_uint16)
        .map(u32::from)
        .map(Length_u32::new::<meter>);

    lap.calories = field_map.get("total_calories").and_then(map_uint16);

    lap.distance = field_map
        .get("total_distance")
        .and_then(map_float64)
        .map(Length_f64::new::<meter>);

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
}
