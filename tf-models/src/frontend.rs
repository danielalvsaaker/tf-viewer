mod lap;
mod record;
mod session;

mod gear;
mod totals;

pub use lap::Lap;
pub use record::Record;
pub use session::Session;
pub use totals::Totals;

pub use gear::Gear;

use super::backend;
use crate::Unit;

#[derive(serde::Serialize)]
pub struct Activity<'a> {
    pub id: String,
    pub gear_id: Option<String>,
    pub session: Session<'a>,
    pub record: Record<'a>,
    pub lap: Vec<Lap<'a>>,
}

impl<'a> Activity<'a> {
    pub fn from_backend(
        id: String,
        gear_id: Option<String>,
        session: backend::Session,
        record: backend::Record,
        lap: Vec<backend::Lap>,
        unit: &Unit,
    ) -> Self {
        Self {
            id,
            gear_id,
            session: Session::from_backend(session, unit),
            record: Record::from_backend(record, unit),
            lap: Lap::from_backend_iter(lap, unit),
        }
    }
}
