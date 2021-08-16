use serde::Serialize;

#[derive(Serialize)]
pub struct Gear {
    pub id: String,
    #[serde(flatten)]
    pub gear: crate::backend::Gear,
}

impl Gear {
    pub fn from_backend(gear: crate::backend::Gear, id: String) -> Self {
        Self { id, gear }
    }
}
