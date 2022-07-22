use super::{ActivityRoot, GearRoot, OAuthGuard};
use crate::connection::{Connection, PageInfo};
use async_graphql::{Context, Object, Result};
use tf_auth::scopes::{self, Read};
use tf_database::{
    error::Error,
    query::{ActivityQuery, UserQuery},
    Database,
};
use tf_models::{activity::Session, gear::Gear, user::User, UserId};

pub struct UserRoot {
    pub query: UserQuery,
}

#[Object(name = "User")]
impl UserRoot {
    async fn id(&self) -> &UserId {
        &self.query.user_id
    }

    #[graphql(flatten)]
    async fn _user(&self, ctx: &Context<'_>) -> Result<User> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || Ok(db.root()?.get(&query)?.unwrap())).await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Activity))")]
    pub(super) async fn activities(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] skip: usize,
        #[graphql(default = 10)] take: usize,
        #[graphql(default)] reverse: bool,
    ) -> Result<Connection<ActivityRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        let (edges, total_count) = tokio::task::spawn_blocking(move || {
            let collection = db.root::<User>()?.traverse::<Session>()?;

            let edges = collection
                .keys(&query, skip, take, reverse)?
                .map(|query| ActivityRoot { query })
                .collect();

            let total_count = collection.count(&query)?;

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
        activity: ActivityQuery,
    ) -> Result<Option<ActivityRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .traverse::<Session>()?
                .contains_key(&activity)?
                .then(|| ActivityRoot { query: activity }))
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
        let query = self.query;

        let (edges, total_count) = tokio::task::spawn_blocking(move || {
            let collection = db.root::<User>()?.traverse::<Gear>()?;

            let edges = collection
                .keys(&query, skip, take, reverse)?
                .map(|query| GearRoot { query })
                .collect();

            let total_count = collection.count(&query)?;

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
