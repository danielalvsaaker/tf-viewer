use async_graphql::{Context, Object, Result};

use super::{OAuthGuard, UserRoot};
use tf_auth::scopes::{self, Read};
use tf_database::{query::GearQuery, Database};
use tf_models::{gear::Gear, user::User, GearId};

pub struct GearRoot {
    pub query: GearQuery,
}

#[Object(name = "Gear")]
impl GearRoot {
    async fn id(&self) -> &GearId {
        &self.query.id
    }

    #[graphql(flatten)]
    async fn _self(&self, ctx: &Context<'_>) -> Result<Gear> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || Ok(db.root::<Gear>()?.get(&query)?.unwrap())).await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<UserRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<Gear>()?
                .traverse::<User>()?
                .get_foreign(&query)?
                .map(|query| UserRoot { query }))
        })
        .await?
    }
}
