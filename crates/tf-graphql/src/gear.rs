use async_graphql::*;

use super::{OAuthGuard, UserRoot};
use tf_auth::scopes::{self, Read};
use tf_database::{query::GearQuery, Database};
use tf_models::{gear::Gear, user::User, GearId};

pub(super) struct GearRoot {
    pub(super) inner: GearQuery,
}

#[Object(name = "Gear")]
impl GearRoot {
    async fn id(&self) -> &GearId {
        &self.inner.id
    }

    #[graphql(flatten)]
    async fn gear(&self, ctx: &Context<'_>) -> Result<Gear> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || Ok(db.root::<Gear>()?.get(&inner)?.unwrap())).await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<UserRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let inner = self.inner;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<Gear>()?
                .traverse::<User>()?
                .get_foreign(&inner)?
                .map(|inner| UserRoot { inner }))
        })
        .await?
    }
}
