mod activity;
mod gear;
mod user;

mod guard;
use guard::OAuthGuard;

use self::{activity::ActivityRoot, gear::GearRoot, user::UserRoot};
use tf_auth::scopes::{self, Read};
use tf_database::{Database, query::{ActivityQuery, UserQuery}};
use tf_models::{ActivityId, UserId, activity::Session, user::User};

use async_graphql::{*};

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn user(&self, ctx: &Context<'_>, user_id: UserId) -> Result<Option<UserRoot>> {
        let inner = UserQuery { user_id };

        Ok(ctx
            .data_unchecked::<Database>()
            .root::<User>()?
            .contains_key(&inner)?
            .then(|| UserRoot { inner }))
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserRoot>> {
        Ok(ctx
            .data_unchecked::<Database>()
            .root::<User>()?
            .iter(0, 25)
            .map(|inner| UserRoot { inner })
            .collect())
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Activity))")]
    async fn activity(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        activity: ActivityId,
    ) -> Result<Option<ActivityRoot>> {
        let inner = ActivityQuery {
            user_id: user,
            id: activity,
        };

        Ok(ctx
            .data_unchecked::<Database>()
            .root::<User>()?
            .traverse::<Session>()?
            .contains_key(&inner)?
            .then(|| ActivityRoot { inner }))
    }
}
