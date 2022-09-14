use crate::{guard::OAuthGuard, query};
use tf_database::Database;
use tf_models::{
    gear::Gear,
    query::{GearQuery, UserQuery},
    user::User,
    GearId, UserId,
};
use tf_scopes::{self as scopes, Write};

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
        user: UserId,
        input: Gear,
    ) -> Result<CreateGearPayload> {
        let db = ctx.data_unchecked::<Database>().clone();

        let gear = GearQuery {
            user_id: user,
            id: GearId::new(),
        };
        let user = UserQuery { user_id: user };

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
        user: UserId,
        gear: GearId,
        input: Gear,
    ) -> Result<UpdateGearPayload> {
        let db = ctx.data_unchecked::<Database>().clone();

        let gear = GearQuery {
            user_id: user,
            id: gear,
        };
        let user = UserQuery { user_id: user };

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
        user: UserId,
        gear: GearId,
    ) -> Result<Option<DeleteGearPayload>> {
        let db = ctx.data_unchecked::<Database>().clone();

        let gear = GearQuery {
            user_id: user,
            id: gear,
        };

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
