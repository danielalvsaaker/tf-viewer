use crate::{
    primitives::{Key, Value},
    query::{ActivityQuery, ClientQuery, GearQuery, UserQuery},
};
use tf_models::{
    activity::{Lap, Record, Session},
    gear::Gear,
    user::User,
};

pub trait Resource: Value {
    const NAME: &'static str;

    type Key: Key;
}

impl Resource for User {
    const NAME: &'static str = "user";

    type Key = UserQuery;
}

impl Resource for Gear {
    const NAME: &'static str = "gear";

    type Key = GearQuery;
}

impl Resource for Session {
    const NAME: &'static str = "session";

    type Key = ActivityQuery;
}

impl Resource for Record {
    const NAME: &'static str = "record";

    type Key = ActivityQuery;
}

impl Resource for Vec<Lap> {
    const NAME: &'static str = "lap";

    type Key = ActivityQuery;
}
