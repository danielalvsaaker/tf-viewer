use crate::guard::OAuthGuard;
use tf_auth::scopes::{self, Write};
use tf_database::{
    query::{GearQuery, UserQuery},
    Database,
};
use tf_models::{gear::Gear, user::User, GearId, UserId};

use async_graphql::{Context, Object, Result, SimpleObject};

#[derive(Default)]
pub struct GearRoot;

#[derive(SimpleObject)]
struct MutateGearPayload {
    #[graphql(flatten)]
    gear: Gear,
}

#[Object]
impl GearRoot {
    #[graphql(guard = "OAuthGuard::new(Write(scopes::Gear))")]
    async fn create_gear(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        gear: Gear,
    ) -> Result<MutateGearPayload> {
        let db = ctx.data_unchecked::<Database>().clone();
        let user_query = UserQuery { user_id: user };
        let query = GearQuery {
            user_id: user,
            id: GearId::new(),
        };

        tokio::task::spawn_blocking(move || {
            db.root::<User>()?
                .traverse::<Gear>()?
                .insert(&query, &gear, &user_query)?;

            Ok(MutateGearPayload { gear })
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
    ) -> Result<MutateGearPayload> {
        let db = ctx.data_unchecked::<Database>().clone();
        let user_query = UserQuery { user_id: user };
        let query = GearQuery {
            user_id: user,
            id: gear,
        };

        tokio::task::spawn_blocking(move || {
            db.root::<User>()?
                .traverse::<Gear>()?
                .insert(&query, &input, &user_query)?;

            Ok(MutateGearPayload { gear: input })
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Write(scopes::Gear))")]
    async fn delete_gear(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        gear: GearId,
    ) -> Result<Option<MutateGearPayload>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = GearQuery {
            user_id: user,
            id: gear,
        };

        tokio::task::spawn_blocking(move || {
            let gear = db.root::<User>()?.traverse::<Gear>()?.remove(&query)?;

            Ok(gear.map(|gear| MutateGearPayload { gear }))
        })
        .await?
    }
}
