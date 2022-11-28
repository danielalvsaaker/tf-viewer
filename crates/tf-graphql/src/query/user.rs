use super::{ActivityRoot, GearRoot, OAuthGuard};
use crate::connection::{Connection, PageInfo};
use async_graphql::{Context, Object, Result};
use tf_database::{
    error::Error,
    query::{ActivityQuery, GearQuery, UserQuery},
    resource::index::DefaultGear,
    Database,
};
use tf_models::{
    activity::Session,
    gear::Gear,
    user::{User, Zones},
    ActivityId, GearId, UserId,
};
use tf_scopes::{self as scopes, Read};

pub struct UserRoot {
    pub query: UserQuery,
}

#[Object(name = "User")]
impl UserRoot {
    async fn id(&self) -> &UserId {
        &self.query.user_id
    }

    #[graphql(flatten)]
    async fn _self(&self, ctx: &Context<'_>) -> Result<User> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || Ok(db.root()?.get(&query)?.unwrap())).await?
    }

    async fn zones(&self, ctx: &Context<'_>) -> Result<Zones> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || Ok(db.root()?.get(&query)?.unwrap_or_default())).await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Activity))")]
    async fn activity(
        &self,
        ctx: &Context<'_>,
        activity: ActivityId,
    ) -> Result<Option<ActivityRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = ActivityQuery {
            user_id: self.query.user_id,
            id: activity,
        };

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .traverse::<Session>()?
                .contains_key(&query)?
                .then_some(ActivityRoot { query }))
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Activity))")]
    pub(super) async fn activity_connection(
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

            let iter = collection.keys(&query, skip, take, reverse)?;
            let total_count = iter.total_count;

            let edges = iter.map(|query| ActivityRoot { query }).collect();

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

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Gear))")]
    async fn default_gear(&self, ctx: &Context<'_>) -> Result<Option<GearRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .traverse::<DefaultGear>()?
                .key(&query)?
                .map(|query| GearRoot { query }))
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Gear))")]
    async fn gear(&self, ctx: &Context<'_>, gear: GearId) -> Result<Option<GearRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = GearQuery {
            user_id: self.query.user_id,
            id: gear,
        };

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .traverse::<Gear>()?
                .contains_key(&query)?
                .then_some(GearRoot { query }))
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Gear))")]
    async fn gear_connection(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 0)] skip: usize,
        #[graphql(default = 10)] take: usize,
        #[graphql(default)] reverse: bool,
    ) -> Result<Connection<GearRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || {
            let collection = db.root::<User>()?.traverse::<Gear>()?;

            let iter = collection.keys(&query, skip, take, reverse)?;

            let total_count = iter.total_count;
            let edges = iter.map(|query| GearRoot { query }).collect();

            Ok(Connection {
                edges,
                total_count,
                page_info: PageInfo {
                    has_previous_page: skip.checked_sub(take).is_some(),
                    has_next_page: (skip + take) < total_count,
                },
            })
        })
        .await?
    }
}
