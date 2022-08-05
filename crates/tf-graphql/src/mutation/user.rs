use crate::{guard::OAuthGuard, query};
use tf_auth::scopes::{self, Write};
use tf_database::{error::Error, resource::index::DefaultGear, Database};
use tf_models::{
    query::{GearQuery, UserQuery},
    user::User,
    GearId, UserId,
};

use async_graphql::{Context, Object, Result, SimpleObject};

#[derive(Default)]
pub struct UserRoot;

#[derive(SimpleObject)]
struct SetDefaultGearPayload {
    user: query::user::UserRoot,
}

#[Object]
impl UserRoot {
    #[graphql(guard = "OAuthGuard::new(Write(scopes::User))")]
    async fn set_default_gear(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        gear: GearId,
    ) -> Result<SetDefaultGearPayload> {
        let db = ctx.data_unchecked::<Database>().clone();

        let user = UserQuery { user_id: user };
        let gear = GearQuery {
            user_id: user.user_id,
            id: gear,
        };

        tokio::task::spawn_blocking(move || {
            db.root::<User>()?
                .traverse::<DefaultGear>()?
                .insert(&user, &gear)?;

            Ok::<_, Error>(())
        })
        .await??;

        Ok(SetDefaultGearPayload {
            user: query::user::UserRoot { query: user },
        })
    }
}
