use fitparser::profile::field_types::MesgNum;
use fitparser::FitDataField;
use fitparser::Value::*;
use std::time::Duration;

use anyhow::{Result, anyhow};
use crate::{Activity, Session, Record, Lap, TimeStamp};



pub fn parse(fit_data: &[u8], gear_id: &str) -> Result<Activity> {
    let mut session: Session = Session::new();
    let mut record: Record = Record::new();
    let mut lap_vec: Vec<Lap> = Vec::new();

    let file = fitparser::from_bytes(fit_data)?;

    if !file.iter().any(|x| x.kind() == MesgNum::Session) {
        return Err(anyhow!("File does not contain session data."))
    }

    for data in file {
        let lap = Lap::new();
        match data.kind() {
            MesgNum::Session => session = parse_session(data.into_vec(), session),
            MesgNum::Record => record = parse_record(data.into_vec(), record),
            MesgNum::Lap => lap_vec.push(parse_lap(data.into_vec(), lap)),
            _ => (),
        }
    }

    Ok(Activity {
            id: session.start_time.0.format("%Y%m%d%H%M").to_string(),
            gear_id: gear_id.to_owned(),
            session,
            record,
            lap: lap_vec,
            }
    )
}


fn parse_session(fields: Vec<FitDataField>, mut session:  Session) -> Session {

    // Semicircle to degree
    let multiplier = 180f64 / 2f64.powi(31);

    for field in fields.iter() {
        match field.name() {
            "avg_cadence" => 
                 session.cadence_avg = match field.value() {
                     UInt8(x) => Some(*x),
                     _ => None,
                 },
            "max_cadence" =>
                 session.cadence_max = match field.value() {
                     UInt8(x) => Some(*x),
                     _ => None,
                 },
            "avg_heart_rate" =>
                 session.heartrate_avg = match field.value() {
                     UInt8(x) => Some(*x),
                     _ => None,
                 },
            "max_heart_rate" =>
                 session.heartrate_max = match field.value() {
                    UInt8(x) => Some(*x),
                    _ => None,
                },
            "enhanced_avg_speed" =>
                 session.speed_avg = match field.value() {
                    Float64(x) => Some((*x * 3.6f64 * 100f64).round() / 100f64),
                    _ => None,
                },
            "enhanced_max_speed" =>
                 session.speed_max = match field.value() {
                    Float64(x) => Some((*x * 3.6f64 * 100f64).round() / 100f64),
                    _ => None,
                },
            "nec_lat" => 
                 session.nec_lat = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                },
            "nec_long" =>
                 session.nec_lon = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                },
            "swc_lat" =>
                 session.swc_lat = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                },
            "swc_long" =>
                 session.swc_lon = match field.value() {
                    SInt32(x) => Some(f64::from(*x) * multiplier),
                    _ => None,
                },
            "num_laps" =>
                 session.laps = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                },
            "sport" =>
                 if let String(x) = field.value() {
                    session.activity_type = x.to_string();
                },
            "total_ascent" =>
                 session.ascent = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                },
            "total_descent" =>
                 session.descent = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                },
            "total_calories" =>
                 if let UInt16(x) = field.value() {
                     session.calories = *x;
                },
            "total_distance" =>
                 session.distance = match field.value() {
                    Float64(x) => Some((*x / 10f64).round() / 100f64),
                    _ => None,
                },
            "total_elapsed_time" =>
                 if let Float64(x) = field.value() {
                     session.duration = Duration::from_secs_f64(*x);
                },
            "total_timer_time" =>
                 if let Float64(x) = field.value() {
                     session.duration_active = Duration::from_secs_f64(*x);
                },
            "start_time" =>
                 if let Timestamp(x) = field.value() {
                     session.start_time = TimeStamp(*x);
                },
            _ => (),
        }
    }

    session
}

fn parse_record(fields: Vec<FitDataField>, mut record: Record) -> Record {

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
            "timestamp" => if let Timestamp(x) = field.value() {
                record.timestamp.push(TimeStamp(*x));
            },
            _ => (),
        }
    }

    if record.timestamp.len() > 1 {
        record.duration.push(chrono::Duration::to_std(&record.timestamp.last().unwrap().0.signed_duration_since(record.timestamp.first().unwrap().0)).unwrap());
    }

    record
}

fn parse_lap(fields: Vec<FitDataField>, mut lap: Lap) -> Lap {

    // Semicircle to degree
    let multiplier = 180f64 / 2f64.powi(31);

    for field in fields.iter() {
        match field.name() {
            "avg_cadence" => lap.cadence_avg = match field.value() {
                UInt8(x) => Some(*x),
                _ => None,
            },
            "max_cadence" => lap.cadence_max = match field.value() {
                UInt8(x) => Some(*x),
                _ => None,
            },
            "avg_heart_rate" => lap.heartrate_avg = match field.value() {
                UInt8(x) => Some(*x),
                _ => None,
            },
            "max_heart_rate" => lap.heartrate_max = match field.value() {
                UInt8(x) => Some(*x),
                _ => None,
            },
            "enhanced_avg_speed" => lap.speed_avg = match field.value() {
                Float64(x) => Some((*x * 3.6f64 * 100f64).round() / 100f64),
                _ => None,
            },
            "enhanced_max_speed" => lap.speed_max = match field.value() {
                Float64(x) => Some((*x * 3.6f64 * 100f64).round() / 100f64),
                _ => None,
            },
            "start_position_lat" => lap.lat_start = match field.value() {
                SInt32(x) => Some(f64::from(*x) * multiplier),
                _ => None,
            },
            "start_position_long" => lap.lon_start = match field.value() {
                SInt32(x) => Some(f64::from(*x) * multiplier),
                _ => None,
            },
            "end_position_lat" => lap.lat_end = match field.value() {
                SInt32(x) => Some(f64::from(*x) * multiplier),
                _ => None,
            },
            "end_position_long" => lap.lon_end = match field.value() {
                SInt32(x) => Some(f64::from(*x) * multiplier),
                _ => None,
            },
            "total_ascent" => lap.ascent = match field.value() {
                UInt16(x) => Some(*x),
                _ => None,
            },
            "total_descent" => lap.descent = match field.value() {
                UInt16(x) => Some(*x),
                _ => None,
            },
            "total_calories" => lap.calories = match field.value() {
                UInt16(x) => Some(*x),
                _ => None,
            },
            "total_distance" => lap.distance = match field.value() {
                Float64(x) => Some((*x / 10f64).round() / 100f64),
                _ => None,
            },
            "total_elapsed_time" => if let Float64(x) = field.value() {
                lap.duration = Duration::from_secs_f64(*x);
            },
            "total_timer_time" => if let Float64(x) = field.value() {
                lap.duration_active = Duration::from_secs_f64(*x);
            },
            _ => (),
        }
    }

    lap
}
