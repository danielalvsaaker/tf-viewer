use async_graphql::MergedObject;

mod activity;
mod gear;
mod user;

use self::{activity::ActivityRoot, gear::GearRoot, user::UserRoot};

#[derive(Default, MergedObject)]
pub struct Mutation(ActivityRoot, GearRoot, UserRoot);
