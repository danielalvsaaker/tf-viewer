use async_graphql::{Context, Object, Result};
use tf_database::{
    query::{GearQuery, UserQuery},
    Database,
};
use tf_models::{gear::Gear, user::User, GearId, UserId};

#[derive(Default)]
pub struct GearRoot;

#[Object]
impl GearRoot {
    async fn create_gear(&self, ctx: &Context<'_>, user: UserId, gear: Gear) -> Result<Gear> {
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

            Ok(gear)
        })
        .await?
    }

    async fn update_gear(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        gear: GearId,
        input: Gear,
    ) -> Result<Gear> {
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

            Ok(input)
        })
        .await?
    }

    async fn delete_gear(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        gear: GearId,
    ) -> Result<Option<Gear>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = GearQuery {
            user_id: user,
            id: gear,
        };

        tokio::task::spawn_blocking(move || {
            Ok(db.root::<User>()?.traverse::<Gear>()?.remove(&query)?)
        })
        .await?
    }
}
