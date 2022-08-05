use super::{
    connection::{Connection, PageInfo},
    guard::OAuthGuard,
};
use tf_auth::scopes::{self, Read};
use tf_database::{error::Error, Database};
use tf_models::{query::UserQuery, user::User, UserId};

use async_graphql::{Context, Object, Result};

pub mod activity;
pub mod gear;
pub mod user;

use self::{activity::ActivityRoot, gear::GearRoot, user::UserRoot};

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn user(&self, ctx: &Context<'_>, user: UserId) -> Result<Option<UserRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();

        let user = UserQuery { user_id: user };

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .contains_key(&user)?
                .then(|| UserRoot { query: user }))
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn users_connection(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] skip: usize,
        #[graphql(default = 10)] take: usize,
        #[graphql(default)] reverse: bool,
    ) -> Result<Connection<UserRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();

        let (edges, total_count) = tokio::task::spawn_blocking(move || {
            let collection = db.root::<User>()?;

            let edges = collection
                .iter(skip, take, reverse)?
                .map(|query| UserRoot { query })
                .collect();

            let total_count = collection.count()?;

            Ok::<_, Error>((edges, total_count))
        })
        .await??;

        Ok(Connection {
            edges,
            total_count,
            page_info: PageInfo {
                has_previous_page: skip.checked_sub(take).is_some(),
                has_next_page: (skip + take) < total_count,
            },
        })
    }
}
