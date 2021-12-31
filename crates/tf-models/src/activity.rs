use serde::{Deserialize, Serialize};
use crate::Sport;

use uom::si::{
    f64::{Length as Length_f64, Velocity},
    u32::Length as Length_u32,
    u16::Power,
};

use chrono::{DateTime, Local};
use std::time::Duration;

#[derive(Serialize)]
pub struct Activity {
    pub id: String,
    pub gear_id: Option<String>,
    pub session: Session,
    pub record: Record,
    pub lap: Vec<Lap>,
    pub notes: Option<String>,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Session {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<Velocity>,
    pub speed_max: Option<Velocity>,
    pub power_avg: Option<Power>,
    pub power_max: Option<Power>,
    pub nec_lat: Option<f64>,
    pub nec_lon: Option<f64>,
    pub swc_lat: Option<f64>,
    pub swc_lon: Option<f64>,
    pub laps: Option<u16>,
    pub sport: Sport,
    pub ascent: Option<Length_u32>,
    pub descent: Option<Length_u32>,
    pub calories: Option<u16>,
    pub distance: Option<Length_f64>,
    pub duration: Duration,
    pub duration_active: Duration,
    pub start_time: DateTime<Local>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Record {
    pub cadence: Vec<Option<u8>>,
    pub distance: Vec<Option<Length_f64>>,
    pub altitude: Vec<Option<Length_f64>>,
    pub speed: Vec<Option<Velocity>>,
    pub heartrate: Vec<Option<u8>>,
    pub power: Vec<Option<Power>>,
    pub lat: Vec<Option<f64>>,
    pub lon: Vec<Option<f64>>,
    pub timestamp: Vec<Option<DateTime<Local>>>,
    pub duration: Vec<Duration>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Lap {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<Velocity>,
    pub speed_max: Option<Velocity>,
    pub power_avg: Option<Power>,
    pub power_max: Option<Power>,
    pub lat_start: Option<f64>,
    pub lon_start: Option<f64>,
    pub lat_end: Option<f64>,
    pub lon_end: Option<f64>,
    pub ascent: Option<Length_u32>,
    pub descent: Option<Length_u32>,
    pub calories: Option<u16>,
    pub distance: Option<Length_f64>,
    pub duration: Duration,
    pub duration_active: Duration,
}
