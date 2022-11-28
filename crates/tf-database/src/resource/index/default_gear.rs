use crate::{query::UserQuery, resource::Resource};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DefaultGear;

impl Resource for DefaultGear {
    const NAME: &'static str = "default_gear";

    type Key = UserQuery;
}
