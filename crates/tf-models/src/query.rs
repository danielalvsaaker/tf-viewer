use super::{ActivityId, ClientId, GearId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
pub struct ActivityQuery {
    pub user_id: UserId,
    pub id: ActivityId,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClientQuery {
    pub user_id: UserId,
    pub id: ClientId,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
pub struct GearQuery {
    pub user_id: UserId,
    pub id: GearId,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
pub struct UserQuery {
    pub user_id: UserId,
}

impl std::fmt::Display for UserQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.user_id.fmt(f)
    }
}
