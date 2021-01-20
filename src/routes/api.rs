use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DataRequest {
    pub draw: usize,
    pub start: usize,
    pub length: usize,
    pub column: usize,
    pub dir: String,
}

#[derive(Serialize)]
pub struct DataResponse<T: Serialize> {
    pub draw: usize,
    #[serde(rename = "recordsTotal")]
    pub records_total: usize,
    #[serde(rename = "recordsFiltered")]
    pub records_filtered: usize,
    pub data: Vec<T>,
}

#[derive(Serialize)]
pub struct ActivityData {
    #[serde(with = "date_format")]
    pub date: DateTime<Local>,
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

mod date_format {
    use chrono::{DateTime, Local};
    use serde::{self, Serializer};

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format("%d.%m.%Y %H:%M"));
        serializer.serialize_str(&s)
    }
}
