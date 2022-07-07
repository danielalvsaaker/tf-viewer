use crate::query::{activity::ActivityRoot, gear::GearRoot, user::UserRoot};
use async_graphql::{OutputType, SimpleObject};

#[derive(SimpleObject)]
#[graphql(concrete(name = "ActivityConnection", params(ActivityRoot)))]
#[graphql(concrete(name = "GearConnection", params(GearRoot)))]
#[graphql(concrete(name = "UserConnection", params(UserRoot)))]
pub struct Connection<T: OutputType> {
    pub edges: Vec<T>,
    pub total_count: usize,
    pub page_info: PageInfo,
}

#[derive(SimpleObject)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
}
