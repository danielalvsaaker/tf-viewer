use super::{index::DefaultGear, Resource};
use crate::{
    primitives::{Index, Relation},
    Traverse,
};
use tf_models::{
    activity::{Lap, Record, Session},
    gear::Gear,
    query::{ActivityQuery, GearQuery, UserQuery},
    user::User,
};

impl Resource for User {
    const NAME: &'static str = "user";

    type Key = UserQuery;
}

impl Traverse<Session> for User {
    type Collection = Relation<ActivityQuery, Session, UserQuery, User>;
}

impl Traverse<Record> for User {
    type Collection = Relation<ActivityQuery, Record, UserQuery, User>;
}

impl Traverse<Vec<Lap>> for User {
    type Collection = Relation<ActivityQuery, Vec<Lap>, UserQuery, User>;
}

impl Traverse<Gear> for User {
    type Collection = Relation<GearQuery, Gear, UserQuery, User>;
}

impl Traverse<DefaultGear> for User {
    type Collection = Index<UserQuery, DefaultGear, GearQuery, Gear>;
}
