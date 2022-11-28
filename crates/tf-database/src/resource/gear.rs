use super::Resource;
use crate::{primitives::Relation, Traverse};
use tf_models::{
    activity::Session,
    gear::Gear,
    query::{ActivityQuery, GearQuery, UserQuery},
    user::User,
};

impl Resource for Gear {
    const NAME: &'static str = "gear";

    type Key = GearQuery;
}

impl Traverse<User> for Gear {
    type Collection = Relation<GearQuery, Gear, UserQuery, User>;
}

impl Traverse<Session> for Gear {
    type Collection = Relation<ActivityQuery, Session, GearQuery, Gear>;
}
