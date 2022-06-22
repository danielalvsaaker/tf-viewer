use async_graphql::{Context, Object, Result};

use super::{GearRoot, OAuthGuard, UserRoot};
use tf_auth::scopes::{self, Read};
use tf_database::{query::ActivityQuery, Database};
use tf_models::{
    activity::{Lap, Record, Session},
    gear::Gear,
    user::User,
    ActivityId,
};

pub(super) struct ActivityRoot {
    pub(super) inner: ActivityQuery,
}

#[Object(name = "Activity")]
impl ActivityRoot {
    async fn id(&self) -> &ActivityId {
        &self.inner.id
    }

    async fn prev(&self, ctx: &Context<'_>) -> Result<Option<Self>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<Session>()?
                .prev(&inner)?
                .map(|inner| Self { inner }))
        })
        .await?
    }

    async fn next(&self, ctx: &Context<'_>) -> Result<Option<Self>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<Session>()?
                .next(&inner)?
                .map(|inner| Self { inner }))
        })
        .await?
    }

    async fn session(&self, ctx: &Context<'_>) -> Result<Session> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || Ok(db.root::<Session>()?.get(&inner)?.unwrap())).await?
    }

    async fn record(&self, ctx: &Context<'_>) -> Result<Record> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || Ok(db.root::<Record>()?.get(&inner)?.unwrap())).await?
    }

    async fn lap(&self, ctx: &Context<'_>) -> Result<Vec<Lap>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || Ok(db.root::<Vec<Lap>>()?.get(&inner)?.unwrap()))
            .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Gear))")]
    async fn gear(&self, ctx: &Context<'_>) -> Result<Option<GearRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<Session>()?
                .traverse::<Gear>()?
                .get_foreign(&inner)?
                .map(|inner| GearRoot { inner }))
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<UserRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<Session>()?
                .traverse::<User>()?
                .get_foreign(&inner)?
                .map(|inner| UserRoot { inner }))
        })
        .await?
    }
}
