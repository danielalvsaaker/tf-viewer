use async_graphql::*;
use crate::{
    user::User,
};
use tf_database::{
    Database,
    query::UserQuery,
    query::ActivityQuery,
};

struct Activity<'a> {
    inner: ActivityQuery<'a>,
}

#[Object]
impl User {
    fn activities(
        &self,
        ctx: &Context<'_>,
        skip: usize,
        take: usize,
    ) -> Vec<Activity<'_>>
        let query = UserQuery::new(&self.username);
        ctx.data_unchecked::<Database>()
            .activity
            .session
            .keys(&query)
            .skip(skip)
            .take(take)
            .map(|inner| Activity { inner })
            .collect()
}

