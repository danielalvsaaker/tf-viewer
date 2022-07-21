use async_graphql::MergedObject;

mod gear;
use gear::GearRoot;

#[derive(Default, MergedObject)]
pub struct Mutation(GearRoot);
