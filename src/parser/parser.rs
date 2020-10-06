use fitparser::profile::field_types::MesgNum;
use std::string::String;
use fitparser::{FitDataField, ValueWithUnits};
use std::fs::File;
use fitparser::Value::*;
use std::io::prelude::*;
use chrono::offset::Local;
use chrono::DateTime;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TimeStamp(DateTime<Local>);

impl Default for TimeStamp {
    fn default() -> TimeStamp {
        TimeStamp(Local::now())
    }
}


#[derive(Default, Serialize, Deserialize)]
pub struct Session {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<f64>,
    pub speed_max: Option<f64>,
    pub nec_lat: Option<i32>,
    pub nec_lon: Option<i32>,
    pub swc_lat: Option<i32>,
    pub swc_lon: Option<i32>,
    pub laps: Option<u16>,
    pub activity_type: Option<String>,
    pub ascent: Option<u16>,
    pub descent: Option<u16>,
    pub calories: Option<u16>,
    pub distance: Option<f64>,
    pub duration: Option<f64>,
    pub duration_active: Option<f64>,
    pub start_time: Option<TimeStamp>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Record {
    pub cadence: Vec<Option<u8>>,
    pub distance: Vec<Option<f64>>,
    pub altitude: Vec<Option<f64>>,
    pub speed: Vec<Option<f64>>,
    pub heartrate: Vec<Option<u8>>,
    pub lat: Vec<Option<i32>>,
    pub lon: Vec<Option<i32>>,
    pub timestamp: Vec<Option<TimeStamp>>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Lap {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<f64>,
    pub speed_max: Option<f64>,
    pub lat_start: Option<i32>,
    pub lon_start: Option<i32>,
    pub lat_end: Option<i32>,
    pub lon_end: Option<i32>,
    pub ascent: Option<u16>,
    pub descent: Option<u16>,
    pub calories: Option<u16>,
    pub distance: Option<f64>,
    pub duration: Option<f64>,
    pub duration_active: Option<f64>,
}

impl Session {
    fn new() -> Self {
        Default::default()
    }
}

impl Record {
    fn new() -> Self {
        Default::default()
    }
}

impl Lap {
    fn new() -> Self {
        Default::default()
    }
}


pub fn parse(fit_data: &[u8]) -> crate::models::Activity {

    let mut session: Session = Session::new();
    let mut record: Record = Record::new();
    let mut lap_vec: Vec<Lap> = Vec::new();

    for data in fitparser::from_bytes(fit_data).expect("Failed to parse fit-file") {
        let mut lap = Lap::new();
        match data.kind() {
            MesgNum::Session => session = parse_session(data.into_vec(), session),
            MesgNum::Record => record = parse_record(data.into_vec(), record),
            MesgNum::Lap => lap_vec.push(parse_lap(data.into_vec(), lap)),
            _ => (),
        }
    }

    crate::models::Activity {
        id: 2u8,
        gear_id: 2u8,
        session: session,
        record: record,
        lap: lap_vec,
    }
}


fn parse_session(fields: Vec<FitDataField>, mut session:  Session) -> Session {

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
                    Float64(x) => Some(*x),
                    _ => None,
                },
            "enhanced_max_speed" =>
                 session.speed_max = match field.value() {
                    Float64(x) => Some(*x),
                    _ => None,
                },
            "nec_lat" => 
                 session.nec_lat = match field.value() {
                    SInt32(x) => Some(*x),
                    _ => None,
                },
            "nec_long" =>
                 session.nec_lon = match field.value() {
                    SInt32(x) => Some(*x),
                    _ => None,
                },
            "swc_lat" =>
                 session.swc_lat = match field.value() {
                    SInt32(x) => Some(*x),
                    _ => None,
                },
            "swc_long" =>
                 session.swc_lon = match field.value() {
                    SInt32(x) => Some(*x),
                    _ => None,
                },
            "num_laps" =>
                 session.laps = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                },
            "sport" =>
                 session.activity_type = match field.value() {
                    String(x) => Some(x.to_string()),
                    _ => None,
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
                 session.calories = match field.value() {
                    UInt16(x) => Some(*x),
                    _ => None,
                },
            "total_distance" =>
                 session.distance = match field.value() {
                    Float64(x) => Some(*x),
                    _ => None,
                },
            "total_elapsed_time" =>
                 session.duration = match field.value() {
                    Float64(x) => Some(*x),
                    _ => None,
                },
            "total_timer_time" =>
                 session.duration_active = match field.value() {
                    Float64(x) => Some(*x),
                    _ => None,
                },
            "start_time" =>
                 session.start_time = match field.value() {
                    Timestamp(x) =>  Some(TimeStamp(*x)),
                    _ => None,
                },
            _ => (),
        }
    }

    session
}

fn parse_record(fields: Vec<FitDataField>, mut record: Record) -> Record {

    for field in fields.iter() {
        match field.name() {
            "cadence" =>  record.cadence.push(match field.value() {
                UInt8(x) => Some(*x),
                _ => None,
            }),
            "distance" =>  record.distance.push(match field.value() {
                Float64(x) => Some(*x),
                _ => None,
            }),
            "enhanced_altitude" =>  record.altitude.push(match field.value() {
                Float64(x) => Some(*x),
                _ => None,
            }),
            "enhanced_speed" => record.speed.push(match field.value() {
                Float64(x) => Some(*x),
                _ => None,
            }),
            "heart_rate" =>  record.heartrate.push(match field.value() {
                UInt8(x) => Some(*x),
                _ => None,
            }),
            "position_lat" =>  record.lat.push(match field.value() {
                SInt32(x) => Some(*x),
                _ => None,
            }),
            "position_long" =>  record.lon.push(match field.value() {
                SInt32(x) => Some(*x),
                _ => None,
            }),
            "timestamp" =>  record.timestamp.push(match field.value() {
                Timestamp(x) => Some(TimeStamp(*x)),
                _ => None,
            }),
            _ => (),
        }
    }

    record
}

fn parse_lap(fields: Vec<FitDataField>, mut lap: Lap) -> Lap {

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
                Float64(x) => Some(*x),
                _ => None,
            },
            "enhanced_max_speed" => lap.speed_max = match field.value() {
                Float64(x) => Some(*x),
                _ => None,
            },
            "start_position_lat" => lap.lat_start = match field.value() {
                SInt32(x) => Some(*x),
                _ => None,
            },
            "start_position_long" => lap.lon_start = match field.value() {
                SInt32(x) => Some(*x),
                _ => None,
            },
            "end_position_lat" => lap.lat_end = match field.value() {
                SInt32(x) => Some(*x),
                _ => None,
            },
            "end_position_long" => lap.lon_end = match field.value() {
                SInt32(x) => Some(*x),
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
                Float64(x) => Some(*x),
                _ => None,
            },
            "total_elapsed_time" => lap.duration = match field.value() {
                Float64(x) => Some(*x),
                _ => None,
            },
            "total_timer_time" => lap.duration_active = match field.value() {
                Float64(x) => Some(*x),
                _ => None,
            },
            _ => (),
        }
    }

    lap
}
                
            



