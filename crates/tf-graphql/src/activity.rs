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

    async fn session(&self, ctx: &Context<'_>) -> Result<Option<Session>> {
        let db = ctx.data_unchecked::<Database>();

        tokio::task::block_in_place(move || Ok(db.root::<Session>()?.get(&self.inner)?))
    }

    async fn record(&self, ctx: &Context<'_>) -> Result<Option<Record>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || Ok(db.root::<Record>()?.get(&inner)?)).await?
    }

    async fn lap(&self, ctx: &Context<'_>) -> Result<Option<Vec<Lap>>> {
        let db = ctx.data_unchecked::<Database>();

        tokio::task::block_in_place(move || Ok(db.root::<Vec<Lap>>()?.get(&self.inner)?))
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::Gear))")]
    async fn gear(&self, ctx: &Context<'_>) -> Result<Option<GearRoot>> {
        let db = ctx.data_unchecked::<Database>();

        tokio::task::block_in_place(move || {
            Ok(db
                .root::<Session>()?
                .traverse::<Gear>()?
                .get_foreign(&self.inner)?
                .map(|inner| GearRoot { inner }))
        })
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<UserRoot>> {
        let db = ctx.data_unchecked::<Database>();

        tokio::task::block_in_place(move || {
            Ok(db
                .root::<Session>()?
                .traverse::<User>()?
                .get_foreign(&self.inner)?
                .map(|inner| UserRoot { inner }))
        })
    }
}
