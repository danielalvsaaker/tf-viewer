use super::{ActivityRoot, GearRoot, OAuthGuard};
use async_graphql::{Context, Object, Result};
use tf_auth::scopes::{self, Read};
use tf_database::{Database, query::{ActivityQuery, UserQuery}};
use tf_models::{user::User, ActivityId, UserId, gear::Gear, activity::Session};

pub struct UserRoot {
    pub(super) inner: UserQuery,
}

#[Object(name = "User")]
impl UserRoot {
    async fn id(&self) -> &UserId {
        &self.inner.user_id
    }

    #[graphql(flatten)]
    async fn _user(&self, ctx: &Context<'_>) -> Result<User> {
        Ok(ctx
            .data_unchecked::<Database>()
            .root()?
            .get(&self.inner)?
            .unwrap())
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Activity))")]
    async fn activities(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] skip: usize,
        #[graphql(default = 10)] take: usize,
    ) -> Result<Vec<ActivityRoot>> {
        Ok(ctx
            .data_unchecked::<Database>()
            .root::<User>()?
            .traverse::<Session>()?
            .keys(&self.inner, skip, take)
            //.skip(skip)
            //.take(take)
            .map(|inner| ActivityRoot { inner })
            .collect())
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Activity))")]
    async fn activity(
        &self,
        ctx: &Context<'_>,
        activity_id: ActivityId,
    ) -> Result<Option<ActivityRoot>> {
        let inner = ActivityQuery {
            user_id: self.inner.user_id,
            id: activity_id,
        };

        Ok(ctx
            .data_unchecked::<Database>()
            .root::<User>()?
            .traverse::<Session>()?
            .contains_key(&inner)?
            .then(|| ActivityRoot { inner }))
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Gear))")]
    async fn gear(&self, ctx: &Context<'_>) -> Result<Vec<GearRoot>> {
        Ok(ctx
            .data_unchecked::<Database>()
            .root::<User>()?
            .traverse::<Gear>()?
            .keys(&self.inner, 0, 25)
            .map(|inner| GearRoot { inner })
            .collect())
    }
}
