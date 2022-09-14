use crate::{guard::OAuthGuard, query};
use tf_database::{error::Error, Database};
use tf_models::{
    activity::{Lap, Record, Session},
    gear::Gear,
    query::{ActivityQuery, GearQuery},
    user::User,
    ActivityId, GearId, UserId,
};
use tf_scopes::{self as scopes, Write};

use async_graphql::{Context, Object, Result, SimpleObject};

#[derive(Default)]
pub struct ActivityRoot;

#[derive(SimpleObject)]
struct LinkGearPayload {
    activity: query::activity::ActivityRoot,
}

#[derive(SimpleObject)]
struct UnlinkGearPayload {
    activity: query::activity::ActivityRoot,
}

#[derive(SimpleObject)]
struct DeleteActivityPayload {
    id: ActivityId,
}

#[Object]
impl ActivityRoot {
    #[graphql(guard = "OAuthGuard::new(Write(scopes::Activity))")]
    async fn link_gear(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        activity: ActivityId,
        gear: GearId,
    ) -> Result<LinkGearPayload> {
        let db = ctx.data_unchecked::<Database>().clone();

        let activity = ActivityQuery {
            user_id: user,
            id: activity,
        };
        let gear = GearQuery {
            user_id: user,
            id: gear,
        };

        tokio::task::spawn_blocking(move || {
            db.root::<User>()?
                .traverse::<Session>()?
                .traverse::<Gear>(&activity)?
                .link(&activity, &gear)?;

            Ok::<_, Error>(())
        })
        .await??;

        Ok(LinkGearPayload {
            activity: query::activity::ActivityRoot { query: activity },
        })
    }

    #[graphql(guard = "OAuthGuard::new(Write(scopes::Activity))")]
    async fn unlink_gear(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        activity: ActivityId,
    ) -> Result<UnlinkGearPayload> {
        let db = ctx.data_unchecked::<Database>().clone();

        let activity = ActivityQuery {
            user_id: user,
            id: activity,
        };

        tokio::task::spawn_blocking(move || {
            db.root::<User>()?
                .traverse::<Session>()?
                .traverse::<Gear>(&activity)?
                .unlink(&activity)?;

            Ok::<_, Error>(())
        })
        .await??;

        Ok(UnlinkGearPayload {
            activity: query::activity::ActivityRoot { query: activity },
        })
    }

    #[graphql(guard = "OAuthGuard::new(Write(scopes::Activity))")]
    async fn delete_activity(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        activity: ActivityId,
    ) -> Result<Option<DeleteActivityPayload>> {
        let db = ctx.data_unchecked::<Database>().clone();

        let activity = ActivityQuery {
            user_id: user,
            id: activity,
        };

        tokio::task::spawn_blocking(move || {
            let root = db.root::<User>()?;

            let session = root.traverse::<Session>()?.remove(&activity)?;
            let record = root.traverse::<Record>()?.remove(&activity)?;
            let lap = root.traverse::<Vec<Lap>>()?.remove(&activity)?;

            Ok(session
                .and(record)
                .and(lap)
                .is_some()
                .then(|| DeleteActivityPayload { id: activity.id }))
        })
        .await?
    }
}
