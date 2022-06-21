use async_graphql::{OutputType, SimpleObject};

#[derive(SimpleObject)]
#[graphql(concrete(name = "ActivityConnection", params(super::ActivityRoot)))]
#[graphql(concrete(name = "GearConnection", params(super::GearRoot)))]
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
