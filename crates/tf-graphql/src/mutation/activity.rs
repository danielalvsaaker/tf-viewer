use crate::guard::OAuthGuard;
use tf_auth::scopes::{self, Write};
use tf_database::{query::ActivityQuery, Database};
use tf_models::{
    activity::{Lap, Record, Session},
    user::User,
    ActivityId, UserId,
};

use async_graphql::{Context, Object, Result, SimpleObject};

#[derive(Default)]
pub struct ActivityRoot;

#[derive(SimpleObject)]
struct MutateActivityPayload {
    session: Session,
    record: Record,
    lap: Vec<Lap>,
}

#[Object]
impl ActivityRoot {
    #[graphql(guard = "OAuthGuard::new(Write(scopes::Activity))")]
    async fn delete_activity(
        &self,
        ctx: &Context<'_>,
        user: UserId,
        activity: ActivityId,
    ) -> Result<Option<MutateActivityPayload>> {
        let db = ctx.data_unchecked::<Database>().clone();
        let query = ActivityQuery {
            user_id: user,
            id: activity,
        };

        tokio::task::spawn_blocking(move || {
            let root = db.root::<User>()?;

            let session = root.traverse::<Session>()?.remove(&query)?;
            let record = root.traverse::<Record>()?.remove(&query)?;
            let lap = root.traverse::<Vec<Lap>>()?.remove(&query)?;

            Ok(session
                .zip(record)
                .zip(lap)
                .map(|((session, record), lap)| MutateActivityPayload {
                    session,
                    record,
                    lap,
                }))
        })
        .await?
    }
}
