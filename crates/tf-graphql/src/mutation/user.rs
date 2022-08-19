use crate::{guard::OAuthGuard, query};
use async_graphql::{Context, Object, Result, SimpleObject};
use oxide_auth::primitives::grant::Grant;
use tf_auth::scopes::{self, Write};
use tf_database::{error::Error, resource::index::DefaultGear, Database};
use tf_models::{
    query::{GearQuery, UserQuery},
    user::User,
    GearId, UserId,
};

#[derive(Default)]
pub struct UserRoot;

#[derive(SimpleObject)]
struct CreateUserPayload {
    user: query::user::UserRoot,
}

#[derive(SimpleObject)]
struct UpdateUserPayload {
    user: query::user::UserRoot,
}

#[derive(SimpleObject)]
struct SetDefaultGearPayload {
    user: query::user::UserRoot,
}

#[Object]
impl UserRoot {
    #[graphql(guard = "OAuthGuard::new(Write(scopes::User))")]
    async fn create_user(&self, ctx: &Context<'_>, input: User) -> Result<CreateUserPayload> {
        let db = ctx.data_unchecked::<Database>().clone();
        let &Grant { ref owner_id, .. } = ctx.data_unchecked::<Grant>();

        let user = UserQuery {
            user_id: owner_id.parse()?,
        };

        tokio::task::spawn_blocking({
            move || {
                let collection = db.root::<User>()?;

                if !collection.contains_key(&user)? {
                    collection.insert(&user, &input)?;
                }

                Ok(CreateUserPayload {
                    user: query::user::UserRoot { query: user },
                })
            }
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Write(scopes::User))")]
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        input: User,
    ) -> Result<UpdateUserPayload> {
        let db = ctx.data_unchecked::<Database>().clone();

        let user = UserQuery { user_id: user };

        tokio::task::spawn_blocking(move || {
            let collection = db.root::<User>()?;

            if collection.contains_key(&user)? {
                collection.insert(&user, &input)?;
            }

            Ok(UpdateUserPayload {
                user: query::user::UserRoot { query: user },
            })
        })
        .await?
    }

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
