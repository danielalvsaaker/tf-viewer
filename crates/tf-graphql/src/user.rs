use super::{
    connection::{Connection, PageInfo},
    ActivityRoot, GearRoot, OAuthGuard,
};
use async_graphql::{Context, Object, Result};
use tf_auth::scopes::{self, Read};
use tf_database::{
    error::Error,
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
        #[graphql(default)] reverse: bool,
    ) -> Result<Connection<ActivityRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        let (edges, total_count) = tokio::task::spawn_blocking(move || {
            let collection = db.root::<User>()?.traverse::<Session>()?;

            let edges = collection
                .keys(&inner, skip, take, reverse)?
                .map(|inner| ActivityRoot { inner })
                .collect();

            let total_count = collection.count(&inner)?;

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
        #[graphql(default)] reverse: bool,
    ) -> Result<Connection<GearRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        let (edges, total_count) = tokio::task::spawn_blocking(move || {
            let collection = db.root::<User>()?.traverse::<Gear>()?;

            let edges = collection
                .keys(&inner, skip, take, reverse)?
                .map(|inner| GearRoot { inner })
                .collect();

            let total_count = collection.count(&inner)?;

            Ok::<_, Error>((edges, total_count))
        })
        .await??;

        Ok(Connection {
            edges,
            total_count,
            page_info: PageInfo {
                has_previous_page: (skip - take) > 0,
                has_next_page: (skip + take) < total_count,
            },
        })
    }
}
