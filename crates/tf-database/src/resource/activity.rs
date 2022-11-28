use super::Resource;
use crate::{primitives::Relation, Traverse};
use tf_models::{
    activity::{Lap, Record, Session},
    gear::Gear,
    query::{ActivityQuery, GearQuery, UserQuery},
    user::User,
};

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

impl Traverse<Gear> for Session {
    type Collection = Relation<ActivityQuery, Session, GearQuery, Gear>;
}

impl Traverse<User> for Session {
    type Collection = Relation<ActivityQuery, Session, UserQuery, User>;
}
