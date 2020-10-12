use serde::{Serialize, Deserialize};
use chrono::offset::Local;
use chrono::DateTime;

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub gear_id: String,
    pub session: Session,
    pub record: Record,
    pub lap: Vec<Lap>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct TimeStamp(pub DateTime<Local>);

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
    pub nec_lat: Option<f64>,
    pub nec_lon: Option<f64>,
    pub swc_lat: Option<f64>,
    pub swc_lon: Option<f64>,
    pub laps: Option<u16>,
    pub activity_type: String,
    pub ascent: Option<u16>,
    pub descent: Option<u16>,
    pub calories: u16,
    pub distance: Option<f64>,
    pub duration: f64,
    pub duration_active: Option<f64>,
    pub start_time: TimeStamp,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Record {
    pub cadence: Vec<Option<u8>>,
    pub distance: Vec<Option<f64>>,
    pub altitude: Vec<Option<f64>>,
    pub speed: Vec<Option<f64>>,
    pub heartrate: Vec<Option<u8>>,
    pub lat: Vec<Option<f64>>,
    pub lon: Vec<Option<f64>>,
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
    pub lat_start: Option<f64>,
    pub lon_start: Option<f64>,
    pub lat_end: Option<f64>,
    pub lon_end: Option<f64>,
    pub ascent: Option<u16>,
    pub descent: Option<u16>,
    pub calories: Option<u16>,
    pub distance: Option<f64>,
    pub duration: Option<f64>,
    pub duration_active: Option<f64>,
}

impl Session {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Record {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Lap {
    pub fn new() -> Self {
        Default::default()
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Gear {
    pub name: String,
    pub kind: String,
    pub fixed_distance: u64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub heartrate_rest: u32,
    pub heartrate_max: u32,
    pub age: u32,
    pub height: u32,
    pub weight: u32,
    pub standard_gear: String,
}

impl User {
    pub fn new() -> Self {
        Default::default()
    }
}
