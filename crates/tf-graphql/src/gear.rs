use async_graphql::*;

use super::{OAuthGuard, UserRoot};
use tf_auth::scopes::{Read, self};
use tf_database::{Database, query::GearQuery};
use tf_models::{gear::Gear, GearId, user::User};

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
        Ok(ctx.data_unchecked::<Database>()
           .root::<Gear>()?
           .get(&self.inner)?
           .unwrap())
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn owner(&self, ctx: &Context<'_>) -> Result<Option<UserRoot>> {
        Ok(ctx
            .data_unchecked::<Database>()
            .root::<Gear>()?
            .traverse::<User>()?
            .get_foreign(&self.inner)?
            .map(|inner| UserRoot { inner }))
    }
}
