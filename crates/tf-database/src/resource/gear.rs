use super::Resource;
use crate::{primitives::Relation, Traverse};
use tf_models::{
    gear::Gear,
    query::{GearQuery, UserQuery},
    user::User,
};

impl Resource for Gear {
    const NAME: &'static str = "gear";

    type Key = GearQuery;
}

impl Traverse<User> for Gear {
    type Collection = Relation<GearQuery, Gear, UserQuery, User>;
}
