use super::{ActivityRoot, GearRoot, OAuthGuard};
use async_graphql::{Context, Object, Result};
use tf_auth::scopes::{self, Read};
use tf_database::{
    query::{ActivityQuery, UserQuery},
    Database,
};
use tf_models::{activity::Session, gear::Gear, user::User, ActivityId, UserId};

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
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || Ok(db.root()?.get(&inner)?.unwrap())).await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Activity))")]
    async fn activities(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] skip: usize,
        #[graphql(default = 10)] take: usize,
    ) -> Result<Vec<ActivityRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .traverse::<Session>()?
                .keys(&inner, skip, take)?
                .map(|inner| ActivityRoot { inner })
                .collect())
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Activity))")]
    async fn activity(
        &self,
        ctx: &Context<'_>,
        activity_id: ActivityId,
    ) -> Result<Option<ActivityRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = ActivityQuery {
            user_id: self.inner.user_id,
            id: activity_id,
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

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Gear))")]
    async fn gear(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] skip: usize,
        #[graphql(default = 10)] take: usize,
    ) -> Result<Vec<GearRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .traverse::<Gear>()?
                .keys(&inner, skip, take)?
                .map(|inner| GearRoot { inner })
                .collect())
        })
        .await?
    }
}
