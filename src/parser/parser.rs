use fitparser::profile::field_types::MesgNum;
use fitparser::FitDataField;
use fitparser::Value::*;
use std::{str::FromStr, string::String};

use crate::{
    error::{Error, ErrorKind, Result},
    models::{Activity, ActivityType, Duration, Lap, Record, Session, TimeStamp},
};

pub fn parse(fit_data: &[u8], gear_id: Option<String>) -> Result<Activity> {
    let mut session: Session = Session::default();
    let mut record: Record = Record::default();
    let mut lap_vec: Vec<Lap> = Vec::new();

    let file = fitparser::from_bytes(fit_data)
        .map_err(|_| Error::BadRequest(ErrorKind::BadRequest, "File is not a valid .fit-file"))?;

    if !file.iter().any(|x| x.kind() == MesgNum::Session) {
        return Err(Error::BadRequest(
            ErrorKind::BadRequest,
            "File does not contain session data.",
        ));
    }

    for data in file {
        match data.kind() {
            MesgNum::Session => parse_session(data.into_vec(), &mut session)?,
            MesgNum::Record => parse_record(data.into_vec(), &mut record)?,
            MesgNum::Lap => {
                let mut lap = Lap::default();
                parse_lap(data.into_vec(), &mut lap)?;
                lap_vec.push(lap);
            }
            _ => (),
        }
    }

    // Cargo fmt wanted it this way, but it's oddly vertical...
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

fn parse_session(fields: Vec<FitDataField>, session: &mut Session) -> Result<()> {
    // Semicircle to degree
    let multiplier = 180f64 / 2f64.powi(31);

    for field in fields.iter() {
        match field.name() {
            "avg_cadence" => {
                session.cadence_avg = match field.value() {
                    UInt8(x) => Some(*x),
                    _ => None,
                }
            }
            "max_cadence" => {
                session.cadence_max = match field.value() {
                    UInt8(x) => Some(*x),
                    _ => None,
                }
            }
            "avg_heart_rate" => {
                session.heartrate_avg = match field.value() {
                    UInt8(x) => Some(*x),
                    _ => None,
                }
            }
            "max_heart_rate" => {
                session.heartrate_max = match field.value() {
                    UInt8(x) => Some(*x),
                    _ => None,
                }
            }
            "enhanced_avg_speed" => {
                session.speed_avg = match field.value() {
                    Float64(x) => Some((*x * 3.6f64 * 100f64).round() / 100f64),
                    _ => None,
                }
            }
            "enhanced_max_speed" => {
                session.speed_max = match field.value() {
                    Float64(x) => Some((*x * 3.6f64 * 100f64).round() / 100f64),
                    _ => None,
                }
            }
            "avg_power" => {
                session.power_avg = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                }
            }
            "max_power" => {
                session.power_max = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                }
            }
            "nec_lat" => {
                session.nec_lat = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                }
            }
            "nec_long" => {
                session.nec_lon = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                }
            }
            "swc_lat" => {
                session.swc_lat = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                }
            }
            "swc_long" => {
                session.swc_lon = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                }
            }
            "num_laps" => {
                session.laps = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                }
            }
            "sport" => {
                if let String(x) = field.value() {
                    session.activity_type = ActivityType::from_str(x)
                        .map_err(|_| Error::BadServerResponse("Failed to parse activity type"))?;
                }
            }
            "total_ascent" => {
                session.ascent = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                }
            }
            "total_descent" => {
                session.descent = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                }
            }
            "total_calories" => {
                if let UInt16(x) = field.value() {
                    session.calories = *x;
                }
            }
            "total_distance" => {
                session.distance = match field.value() {
                    Float64(x) => Some((*x / 10f64).round() / 100f64),
                    _ => None,
                }
            }
            "total_elapsed_time" => {
                if let Float64(x) = field.value() {
                    session.duration = Duration::from_secs_f64(*x);
                }
            }
            "total_timer_time" => {
                if let Float64(x) = field.value() {
                    session.duration_active = Duration::from_secs_f64(*x);
                }
            }
            "start_time" => {
                if let Timestamp(x) = field.value() {
                    session.start_time = TimeStamp(*x);
                }
            }
            _ => (),
        }
    }

    Ok(())
}

fn parse_record(fields: Vec<FitDataField>, record: &mut Record) -> Result<()> {
    // Semicircle to degree
    let multiplier = 180f64 / 2f64.powi(31);

    for field in fields.iter() {
        match field.name() {
            "cadence" => record.cadence.push(match field.value() {
                UInt8(x) => Some(*x),
                _ => None,
            }),
            "distance" => record.distance.push(match field.value() {
                Float64(x) => Some((*x / 10f64).round() / 100f64),
                _ => None,
            }),
            "enhanced_altitude" => record.altitude.push(match field.value() {
                Float64(x) => Some(*x),
                _ => None,
            }),
            "enhanced_speed" => record.speed.push(match field.value() {
                Float64(x) => Some((*x * 3.6f64 * 100f64).round() / 100f64),
                _ => None,
            }),
            "power" => record.power.push(match field.value() {
                UInt16(x) => Some(*x),
                _ => None,
            }),
            "heart_rate" => record.heartrate.push(match field.value() {
                UInt8(x) => Some(*x),
                _ => None,
            }),
            "position_lat" => record.lat.push(match field.value() {
                SInt32(x) => Some(f64::from(*x) * multiplier),
                _ => None,
            }),
            "position_long" => record.lon.push(match field.value() {
                SInt32(x) => Some(f64::from(*x) * multiplier),
                _ => None,
            }),
            "timestamp" => {
                if let Timestamp(x) = field.value() {
                    let timestamp = TimeStamp(*x);

                    match record.timestamp.first() {
                        Some(x) => {
                            let duration = Duration::between(&timestamp, x);
                            record.duration.push(duration);
                        }
                        None => record.duration.push(Duration::default()),
                    }

                    record.timestamp.push(timestamp);
                }
            }
            _ => (),
        }
    }

    Ok(())
}

fn parse_lap(fields: Vec<FitDataField>, lap: &mut Lap) -> Result<()> {
    // Semicircle to degree
    let multiplier = 180f64 / 2f64.powi(31);

    for field in fields.iter() {
        match field.name() {
            "avg_cadence" => {
                lap.cadence_avg = match field.value() {
                    UInt8(x) => Some(*x),
                    _ => None,
                }
            }
            "max_cadence" => {
                lap.cadence_max = match field.value() {
                    UInt8(x) => Some(*x),
                    _ => None,
                }
            }
            "avg_heart_rate" => {
                lap.heartrate_avg = match field.value() {
                    UInt8(x) => Some(*x),
                    _ => None,
                }
            }
            "max_heart_rate" => {
                lap.heartrate_max = match field.value() {
                    UInt8(x) => Some(*x),
                    _ => None,
                }
            }
            "enhanced_avg_speed" => {
                lap.speed_avg = match field.value() {
                    Float64(x) => Some((*x * 3.6f64 * 100f64).round() / 100f64),
                    _ => None,
                }
            }
            "enhanced_max_speed" => {
                lap.speed_max = match field.value() {
                    Float64(x) => Some((*x * 3.6f64 * 100f64).round() / 100f64),
                    _ => None,
                }
            }
            "avg_power" => {
                lap.power_avg = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                }
            }
            "max_power" => {
                lap.power_max = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                }
            }
            "start_position_lat" => {
                lap.lat_start = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                }
            }
            "start_position_long" => {
                lap.lon_start = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                }
            }
            "end_position_lat" => {
                lap.lat_end = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                }
            }
            "end_position_long" => {
                lap.lon_end = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                }
            }
            "total_ascent" => {
                lap.ascent = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                }
            }
            "total_descent" => {
                lap.descent = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                }
            }
            "total_calories" => {
                lap.calories = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                }
            }
            "total_distance" => {
                lap.distance = match field.value() {
                    Float64(x) => Some((*x / 10f64).round() / 100f64),
                    _ => None,
                }
            }
            "total_elapsed_time" => {
                if let Float64(x) = field.value() {
                    lap.duration = Duration::from_secs_f64(*x);
                }
            }
            "total_timer_time" => {
                if let Float64(x) = field.value() {
                    lap.duration_active = Duration::from_secs_f64(*x);
                }
            }
            _ => (),
        }
    }

    Ok(())
}
