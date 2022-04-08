mod activity;
mod gear;
mod user;

mod guard;
use guard::OAuthGuard;

use self::{activity::ActivityRoot, gear::GearRoot, user::UserRoot};
use tf_auth::scopes::{self, Read};
use tf_database::{
    query::{ActivityQuery, UserQuery},
    Database,
};
use tf_models::{activity::Session, user::User, ActivityId, UserId};

use async_graphql::*;

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn user(&self, ctx: &Context<'_>, user_id: UserId) -> Result<Option<UserRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = UserQuery { user_id };

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .contains_key(&inner)?
                .then(|| UserRoot { inner }))
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn users(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] skip: usize,
        #[graphql(default = 10)] take: usize,
    ) -> Result<Vec<UserRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .iter(skip, take)?
                .map(|inner| UserRoot { inner })
                .collect())
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Activity))")]
    async fn activity(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        activity: ActivityId,
    ) -> Result<Option<ActivityRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = ActivityQuery {
            user_id: user,
            id: activity,
        };

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .traverse::<Session>()?
                .contains_key(&inner)?
                .then(|| ActivityRoot { inner }))
        })
        .await?
    }
}
