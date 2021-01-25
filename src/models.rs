use crate::error::{Error, Result};
use chrono::offset::Local;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::{
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub gear_id: Option<String>,
    pub session: Session,
    pub record: Record,
    pub lap: Vec<Lap>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Session {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<f64>,
    pub speed_max: Option<f64>,
    pub power_avg: Option<u16>,
    pub power_max: Option<u16>,
    pub nec_lat: Option<f64>,
    pub nec_lon: Option<f64>,
    pub swc_lat: Option<f64>,
    pub swc_lon: Option<f64>,
    pub laps: Option<u16>,
    pub activity_type: ActivityType,
    pub ascent: Option<u16>,
    pub descent: Option<u16>,
    pub calories: Option<u16>,
    pub distance: Option<f64>,
    pub duration: Duration,
    pub duration_active: Duration,
    pub start_time: TimeStamp,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Record {
    pub cadence: Vec<Option<u8>>,
    pub distance: Vec<Option<f64>>,
    pub altitude: Vec<Option<f64>>,
    pub speed: Vec<Option<f64>>,
    pub heartrate: Vec<Option<u8>>,
    pub power: Vec<Option<u16>>,
    pub lat: Vec<Option<f64>>,
    pub lon: Vec<Option<f64>>,
    pub timestamp: Vec<TimeStamp>,
    pub duration: Vec<Duration>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Lap {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<f64>,
    pub speed_max: Option<f64>,
    pub power_avg: Option<u16>,
    pub power_max: Option<u16>,
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

#[derive(Serialize, Deserialize)]
pub enum ActivityType {
    Running,
    Cycling,
    Other(String),
}

impl ActivityType {
    pub const fn is_running(&self) -> bool {
        matches!(*self, Self::Running)
    }

    pub const fn is_cycling(&self) -> bool {
        matches!(*self, Self::Cycling)
    }
}

impl Default for ActivityType {
    fn default() -> Self {
        Self::Other("Unknown".to_string())
    }
}

impl FromStr for ActivityType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "running" => Self::Running,
            "cycling" => Self::Cycling,
            _ => Self::Other(s.to_string()),
        })
    }
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let activity_type = match self {
            Self::Running => "Running",
            Self::Cycling => "Cycling",
            Self::Other(x) => x,
        };
        write!(f, "{}", activity_type)
    }
}

#[derive(Serialize, Deserialize)]
pub enum GearType {
    RoadBike,
    HybridBike,
    TTBike,
    OffroadBike,
    RunningShoes,
}

impl FromStr for GearType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "road_bike" => Ok(Self::RoadBike),
            "hybrid_bike" => Ok(Self::HybridBike),
            "tt_bike" => Ok(Self::HybridBike),
            "offroad_bike" => Ok(Self::OffroadBike),
            "running_shoes" => Ok(Self::RunningShoes),
            _ => Err(Error::BadServerResponse("Failed to parse gear type")),
        }
    }
}

impl std::fmt::Display for GearType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gear_type = match self {
            Self::RoadBike => "Road bike",
            Self::HybridBike => "Hybrid bike",
            Self::TTBike => "TT bike",
            Self::OffroadBike => "Offroad bike",
            Self::RunningShoes => "Running shoes",
        };
        write!(f, "{}", gear_type)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Gear {
    pub name: String,
    pub gear_type: GearType,
    pub fixed_distance: f64,
}

pub struct UserTotals {
    pub cycling_month: (f64, Duration, usize),
    pub cycling_year: (f64, Duration, usize),
    pub cycling_all: (f64, Duration, usize),
    pub running_month: (f64, Duration, usize),
    pub running_year: (f64, Duration, usize),
    pub running_all: (f64, Duration, usize),
}

/// Wrapper for chrono::DateTime
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeStamp(pub DateTime<Local>);

impl Default for TimeStamp {
    fn default() -> TimeStamp {
        TimeStamp(Local::now())
    }
}

impl std::fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%d.%m.%Y %H:%M"))
    }
}

/// Wrapper for std::time::Duration
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy, Default)]
pub struct Duration(std::time::Duration);

impl Duration {
    pub fn from_secs_f64(secs: f64) -> Self {
        Duration(std::time::Duration::from_secs_f64(secs))
    }

    pub fn between(ts1: &TimeStamp, ts2: &TimeStamp) -> Self {
        Duration(
            chrono::Duration::to_std(&ts1.0.signed_duration_since(ts2.0))
                .expect("Duration out of bounds"),
        )
    }
}

impl Add for Duration {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self::Output {
        Duration(
            self.0
                .checked_add(rhs.0)
                .expect("overflow when adding durations."),
        )
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 = self.0 + rhs.0;
    }
}

impl Sub for Duration {
    type Output = Duration;
    fn sub(self, rhs: Duration) -> Duration {
        Duration(
            self.0
                .checked_sub(rhs.0)
                .expect("overflow when subtracting durations"),
        )
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
