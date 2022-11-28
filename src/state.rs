use crate::{cache::ThumbnailCache, Broker, Schema};
use axum::extract::FromRef;
use tf_auth::{database::Database as AuthDatabase, State as AuthState};
use tf_database::Database as AppDatabase;

#[derive(Clone, FromRef)]
pub struct Database {
    pub main: AppDatabase,
    pub auth: AuthDatabase,
}

impl FromRef<AppState> for AppDatabase {
    fn from_ref(state: &AppState) -> Self {
        Self::from_ref(&state.database)
    }
}

impl FromRef<AppState> for AuthDatabase {
    fn from_ref(state: &AppState) -> Self {
        Self::from_ref(&state.database)
    }
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub broker: Broker,
    pub cache: ThumbnailCache,
    pub state: AuthState,
    pub schema: Schema,
    pub database: Database,
}
