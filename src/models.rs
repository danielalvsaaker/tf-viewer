use chrono::offset::Local;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

/// Wrapper for chrono::DateTime
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct TimeStamp(pub DateTime<Local>);

impl Default for TimeStamp {
    fn default() -> TimeStamp {
        TimeStamp(Local::now())
    }
}

impl std::fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%d.%m.%Y %H:%M").to_string())
    }
}

/// Wrapper for std::time::Duration
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub struct Duration(std::time::Duration);

impl Duration {
    pub fn from_secs_f64(secs: f64) -> Self {
        Duration(std::time::Duration::from_secs_f64(secs))
    }

    pub fn as_secs_f64(&self) -> f64 {
        self.0.as_secs_f64()
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.as_secs();
        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);

        write!(f, "{:02}:{:02}:{:02}", h, m, s)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub gear_id: String,
    pub session: Session,
    pub record: Record,
    pub lap: Vec<Lap>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
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
    pub duration: Duration,
    pub duration_active: Duration,
    pub start_time: TimeStamp,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Record {
    pub cadence: Vec<Option<u8>>,
    pub distance: Vec<Option<f64>>,
    pub altitude: Vec<Option<f64>>,
    pub speed: Vec<Option<f64>>,
    pub heartrate: Vec<Option<u8>>,
    pub lat: Vec<Option<f64>>,
    pub lon: Vec<Option<f64>>,
    pub timestamp: Vec<TimeStamp>,
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
    pub duration: Duration,
    pub duration_active: Duration,
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
    pub fixed_distance: f64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub heartrate_rest: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub standard_gear: String,
}

impl User {
    pub fn new() -> Self {
        Default::default()
    }
}
