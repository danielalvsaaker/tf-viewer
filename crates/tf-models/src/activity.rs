use crate::Sport;
use serde::{Deserialize, Serialize};

use crate::{
    types::{AngularVelocity, DateTime, Duration, Energy, LengthF64, LengthU32, Power, Velocity},
    ActivityId, GearId, UserId,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Activity {
    pub owner: UserId,
    pub id: ActivityId,
    pub gear_id: Option<GearId>,
    pub session: Session,
    pub record: Record,
    pub lap: Vec<Lap>,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct Session {
    pub cadence_avg: Option<AngularVelocity>,
    pub cadence_max: Option<AngularVelocity>,
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
    pub ascent: Option<LengthU32>,
    pub descent: Option<LengthU32>,
    pub calories: Option<Energy>,
    pub distance: Option<LengthF64>,
    pub duration: Duration,
    pub duration_active: Duration,
    pub start_time: DateTime,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct Record {
    pub cadence: Vec<Option<AngularVelocity>>,
    pub distance: Vec<Option<LengthF64>>,
    pub altitude: Vec<Option<LengthF64>>,
    pub speed: Vec<Option<Velocity>>,
    pub heartrate: Vec<Option<u8>>,
    pub power: Vec<Option<Power>>,
    pub lat: Vec<Option<f64>>,
    pub lon: Vec<Option<f64>>,
    pub timestamp: Vec<Option<DateTime>>,
    pub duration: Vec<Duration>,
}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
pub struct Lap {
    pub cadence_avg: Option<AngularVelocity>,
    pub cadence_max: Option<AngularVelocity>,
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
    pub ascent: Option<LengthU32>,
    pub descent: Option<LengthU32>,
    pub calories: Option<Energy>,
    pub distance: Option<LengthF64>,
    pub duration: Duration,
    pub duration_active: Duration,
}
