use serde::{Deserialize, Serialize};
use uom::si::f64::{Length as Length_f64, Velocity};
use uom::si::u16::{Length as Length_u16};


pub struct SessionRef {
    cadence_avg: Option<u8>,
    heartrate_avg: Option<u8>,
    heartrate_max: Option<u8>,
    speed_avg: Option<Value>,
    speed_max: Option<Value>,
    power_avg: Option<u16>,
    power_max: Option<u16>,
    nec_lat: Option<f64>,
    nec_lon: Option<f64>,
    swc_lat: Option<f64>,
    swc_lon: Option<f64>,
    laps: Option<u16>,
    activity_type: ActivityType,
    ascent: Option<Value>,
    descent: Option<Value>,
    calories: Option<u16>,
    distance: Option<Value>,
    duration: Duration,
    duration_active: Duration,
    start_time: TimeStamp,
}

impl SessionRef {
    fn from(s: Session, unit: Unit) -> Self {
        Self {
            cadence_avg: s.cadence_avg,
            heartrate_avg: s.heartrate_avg,
            heartrate_max: s.heartrate_max,
            speed_avg: s.speed_avg,
            speed_max: s.speed_max,
            power_avg: s.power_avg,
            power_max: s.power_max,
            nec_lat: s.nec_lat,
            nec_lon: s.nec_lon,
            swc_lat: s.swc_lat,
            swc_lon: s.swc_lon,
            laps: s.laps,
            activity_type: s.activity_type,
            ascent: s.ascent,
            descent: s.descent,
            calories: s.calories,
            distance: s.distance,
            duration: s.duration,
            duration_active: s.duration_active,
            start_time: s.start_time,
        }
    }
}


