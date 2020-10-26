use serde::{Serialize, Deserialize};
use super::date_format;

#[derive(Deserialize)]
pub struct DataRequest {
    pub draw: usize,
    pub start: usize,
    pub length: usize,
    pub column: usize,
    pub dir: String,
}

#[derive(Serialize, Debug)]
pub struct DataResponse<T: Serialize>{
    pub draw: usize,
    pub recordsTotal: usize,
    pub recordsFiltered: usize,
    pub data: Vec<T>,
}

#[derive(Serialize, Debug)]
pub struct ActivityData {
    #[serde(with = "date_format")]
    pub date: chrono::DateTime<chrono::Local>,
    pub activity_type: String,
    pub duration: String,
    pub distance: Option<f64>,
    pub calories: u16,
    pub cadence_avg: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<f64>,
    pub speed_max: Option<f64>,
    pub ascent: Option<u16>,
    pub descent: Option<u16>,
    pub id: String,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub name: String,
}
