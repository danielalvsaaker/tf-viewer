use async_graphql::{Context, Object, Result};

use super::{ActivityRoot, OAuthGuard, UserRoot};
use crate::connection::{Connection, PageInfo};
use tf_database::{
    query::{GearQuery, UserQuery},
    resource::index::DefaultGear,
    Database,
};
use tf_models::{activity::Session, gear::Gear, user::User, GearId};
use tf_scopes::{self as scopes, Read};

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

    async fn activity_connection(
        &self,
        ctx: &Context<'_>,
        skip: usize,
        take: usize,
        reverse: bool,
    ) -> Result<Connection<ActivityRoot>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;

        tokio::task::spawn_blocking(move || {
            let collection = db.root::<Gear>()?.traverse::<Session>()?;

            let iter = collection.join(&query, skip, take, reverse)?;

            let total_count = iter.total_count;
            let edges = iter.map(|query| ActivityRoot { query }).collect::<Vec<_>>();

            Ok(Connection {
                edges,
                total_count,
                page_info: PageInfo {
                    has_previous_page: skip.checked_sub(take).is_some(),
                    has_next_page: (skip + take) < total_count,
                },
            })
        })
        .await?
    }

    #[graphql(guard = "OAuthGuard::new(Read(scopes::User))")]
    async fn owner(&self) -> UserRoot {
        let query = UserQuery {
            user_id: self.query.user_id,
        };

        UserRoot { query }
    }

    async fn is_default_gear(&self, ctx: &Context<'_>) -> Result<bool> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = self.query;
        let user_query = UserQuery {
            user_id: query.user_id,
        };

        tokio::task::spawn_blocking(move || {
            Ok(db
                .root::<User>()?
                .traverse::<DefaultGear>()?
                .key(&user_query)?
                .map(|default_gear| default_gear == query)
                .unwrap_or_default())
        })
        .await?
    }
}
