use super::{ActivityId, ClientId, GearId, InvalidLengthError, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
pub struct ActivityQuery {
    pub user_id: UserId,
    pub id: ActivityId,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ClientQuery {
    pub user_id: UserId,
    pub id: ClientId,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl std::fmt::Display for ActivityQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.user_id, self.id)
    }
}

impl std::fmt::Display for GearQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.user_id, self.id)
    }
}

impl std::fmt::Display for ClientQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.user_id, self.id)
    }
}

impl std::str::FromStr for ActivityQuery {
    type Err = InvalidLengthError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let user_id = src
            .get(0..UserId::LENGTH)
            .ok_or(InvalidLengthError {
                expected: UserId::LENGTH,
                actual: src.len(),
            })?
            .parse()?;

        let id = src[UserId::LENGTH..].parse()?;

        Ok(Self { user_id, id })
    }
}

impl std::fmt::Display for UserQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.user_id.fmt(f)
    }
}

impl std::str::FromStr for UserQuery {
    type Err = InvalidLengthError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let user_id = src.parse()?;

        Ok(Self { user_id })
    }
}

impl std::str::FromStr for GearQuery {
    type Err = InvalidLengthError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let user_id = src
            .get(0..UserId::LENGTH)
            .ok_or(InvalidLengthError {
                expected: UserId::LENGTH,
                actual: src.len(),
            })?
            .parse()?;

        let id = src[UserId::LENGTH..].parse()?;

        Ok(Self { user_id, id })
    }
}

impl std::str::FromStr for ClientQuery {
    type Err = InvalidLengthError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let user_id = src
            .get(0..UserId::LENGTH)
            .ok_or(InvalidLengthError {
                expected: UserId::LENGTH,
                actual: src.len(),
            })?
            .parse()?;

        let id = src[UserId::LENGTH..].parse()?;

        Ok(Self { user_id, id })
    }
}
