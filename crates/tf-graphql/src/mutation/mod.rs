use async_graphql::MergedObject;

mod activity;
mod gear;

use self::{activity::ActivityRoot, gear::GearRoot};

#[derive(Default, MergedObject)]
pub struct Mutation(ActivityRoot, GearRoot);
