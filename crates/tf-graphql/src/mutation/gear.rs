use crate::{guard::OAuthGuard, query};
use tf_auth::scopes::{self, Write};
use tf_database::Database;
use tf_models::{
    gear::Gear,
    query::{GearQuery, UserQuery},
    user::User,
    GearId,
};

use async_graphql::{Context, Object, Result, SimpleObject};

#[derive(Default)]
pub struct GearRoot;

#[derive(SimpleObject)]
struct CreateGearPayload {
    gear: query::gear::GearRoot,
}

#[derive(SimpleObject)]
struct UpdateGearPayload {
    gear: query::gear::GearRoot,
}

#[derive(SimpleObject)]
struct DeleteGearPayload {
    id: GearId,
}

#[Object]
impl GearRoot {
    #[graphql(guard = "OAuthGuard::new(Write(scopes::Gear))")]
    async fn create_gear(
        &self,
        ctx: &Context<'_>,
        user: UserQuery,
        input: Gear,
    ) -> Result<CreateGearPayload> {
        let db = ctx.data_unchecked::<Database>().clone();
        let gear = GearQuery {
            user_id: user.user_id,
            id: GearId::new(),
        };

        tokio::task::spawn_blocking(move || {
            db.root::<User>()?
                .traverse::<Gear>()?
                .insert(&gear, &input, &user)?;

            Ok(CreateGearPayload {
                gear: query::gear::GearRoot { query: gear },
            })
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Write(scopes::Gear))")]
    async fn update_gear(
        &self,
        ctx: &Context<'_>,
        gear: GearQuery,
        input: Gear,
    ) -> Result<UpdateGearPayload> {
        let db = ctx.data_unchecked::<Database>().clone();
        let user = UserQuery {
            user_id: gear.user_id,
        };

        tokio::task::spawn_blocking(move || {
            db.root::<User>()?
                .traverse::<Gear>()?
                .insert(&gear, &input, &user)?;

            Ok(UpdateGearPayload {
                gear: query::gear::GearRoot { query: gear },
            })
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Write(scopes::Gear))")]
    async fn delete_gear(
        &self,
        ctx: &Context<'_>,
        gear: GearQuery,
    ) -> Result<Option<DeleteGearPayload>> {
        let db = ctx.data_unchecked::<Database>().clone();

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .traverse::<Gear>()?
                .remove(&gear)?
                .is_some()
                .then(|| DeleteGearPayload { id: gear.id }))
        })
        .await?
    }
}
