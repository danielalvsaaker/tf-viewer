use super::{Duration, TimeStamp};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use uom::si::f64::{Length as Length_f64, Velocity};
use uom::si::u16::Length as Length_u16;

pub struct Activity {
    pub id: String,
    pub gear_id: Option<String>,
    pub session: Session,
    pub record: Record, pub lap: Vec<Lap>,
    pub notes: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Session {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<Velocity>,
    pub speed_max: Option<Velocity>,
    pub power_avg: Option<u16>,
    pub power_max: Option<u16>,
    pub nec_lat: Option<f64>,
    pub nec_lon: Option<f64>,
    pub swc_lat: Option<f64>,
    pub swc_lon: Option<f64>,
    pub laps: Option<u16>,
    pub activity_type: ActivityType,
    pub ascent: Option<Length_u16>,
    pub descent: Option<Length_u16>,
    pub calories: Option<u16>,
    pub distance: Option<Length_f64>,
    pub duration: Duration,
    pub duration_active: Duration,
    pub start_time: TimeStamp,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Record {
    pub cadence: Vec<Option<u8>>,
    pub distance: Vec<Option<Length_f64>>,
    pub altitude: Vec<Option<Length_f64>>,
    pub speed: Vec<Option<Velocity>>,
    pub heartrate: Vec<Option<u8>>,
    pub power: Vec<Option<u16>>,
    pub lat: Vec<Option<f64>>,
    pub lon: Vec<Option<f64>>,
    pub timestamp: Vec<TimeStamp>,
    pub duration: Vec<Duration>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Lap {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<Velocity>,
    pub speed_max: Option<Velocity>,
    pub power_avg: Option<u16>,
    pub power_max: Option<u16>,
    pub lat_start: Option<f64>,
    pub lon_start: Option<f64>,
    pub lat_end: Option<f64>,
    pub lon_end: Option<f64>,
    pub ascent: Option<Length_u16>,
    pub descent: Option<Length_u16>,
    pub calories: Option<u16>,
    pub distance: Option<Length_f64>,
    pub duration: Duration,
    pub duration_active: Duration,
}

#[derive(Serialize, Deserialize, Clone, PartialOrd, Ord, PartialEq, Eq)]
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
        let capitalize_truncate = |x: &str| {
            let c = x.split("_").take(2).collect::<Vec<&str>>().join(" ");
            match c.chars().next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        };

        let activity_type = match self {
            Self::Running => "Running".to_string(),
            Self::Cycling => "Cycling".to_string(),
            Self::Other(x) => capitalize_truncate(&x),
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
            "tt_bike" => Ok(Self::TTBike),
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
    pub fixed_distance: Length_f64,
}

pub struct UserTotals {
    pub cycling_month: (Length_f64, Duration, usize),
    pub cycling_year: (Length_f64, Duration, usize),
    pub cycling_all: (Length_f64, Duration, usize),
    pub running_month: (Length_f64, Duration, usize),
    pub running_year: (Length_f64, Duration, usize),
    pub running_all: (Length_f64, Duration, usize),
}
