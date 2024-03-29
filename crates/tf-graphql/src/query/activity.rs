use async_graphql::{Context, Object, Result};

use super::{GearRoot, OAuthGuard, UserRoot};
use tf_database::{query::ActivityQuery, Database};
use tf_models::{
    activity::{Lap, Record, Session},
    gear::Gear,
    user::User,
    ActivityId,
};
use tf_scopes::{self as scopes, Read};

mod record;
use record::RecordRoot;

pub struct ActivityRoot {
    pub query: ActivityQuery,
}

#[Object(name = "Activity")]
impl ActivityRoot {
    async fn id(&self) -> &ActivityId {
        &self.query.id
    }

    async fn prev(&self, ctx: &Context<'_>) -> Result<Option<Self>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<Session>()?
                .prev(&query)?
                .map(|query| Self { query }))
        })
        .await?
    }

    async fn next(&self, ctx: &Context<'_>) -> Result<Option<Self>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<Session>()?
                .next(&query)?
                .map(|query| Self { query }))
        })
        .await?
    }

    async fn session(&self, ctx: &Context<'_>) -> Result<Session> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || Ok(db.root::<Session>()?.get(&query)?.unwrap())).await?
    }

    async fn record(&self, ctx: &Context<'_>) -> Result<RecordRoot> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        let buffer = tokio::task::spawn_blocking(move || {
            Ok::<_, tf_database::error::Error>(db.root::<Record>()?.get_raw(&query)?.unwrap())
        })
        .await??;

        Ok(RecordRoot { buffer })
    }

    async fn lap(&self, ctx: &Context<'_>) -> Result<Vec<Lap>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || Ok(db.root::<Vec<Lap>>()?.get(&query)?.unwrap()))
            .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Gear))")]
    async fn gear(&self, ctx: &Context<'_>) -> Result<Option<GearRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<Session>()?
                .traverse::<Gear>()?
                .get_foreign(&query)?
                .map(|query| GearRoot { query }))
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<UserRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<Session>()?
                .traverse::<User>()?
                .get_foreign(&query)?
                .map(|query| UserRoot { query }))
        })
        .await?
    }
}
