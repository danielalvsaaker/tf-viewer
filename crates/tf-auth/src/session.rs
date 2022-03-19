use async_session::{MemoryStore, Session as CookieSession, SessionStore as _};
use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    headers::Cookie,
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
};

const COOKIE_NAME: &str = "tf_session";

pub struct Session {
    session: Option<CookieSession>,
    store: MemoryStore,
}

impl Session {
    pub async fn remember(&self, user_id: String) -> HeaderMap {
        let mut session = CookieSession::new();
        session.insert("id", user_id).unwrap();
        let cookie = self.store.store_session(session).await.unwrap().unwrap();
        [(
            SET_COOKIE,
            HeaderValue::from_str(&format!("{}={}", COOKIE_NAME, cookie)).unwrap(),
        )]
        .into_iter()
        .collect()
    }

    pub fn id(&self) -> Option<String> {
        self.session.as_ref().and_then(|s| s.get("id"))
    }

    pub async fn forget(&mut self) {
        if let Some(session) = self.session.take() {
            self.store.destroy_session(session).await;
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for Session
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<MemoryStore>::from_request(req).await.unwrap();

        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();

        let session = cookie.as_ref().and_then(|c| c.get(COOKIE_NAME));

        let session = if let Some(s) = session {
            store.load_session(s.into()).await.unwrap()
        } else {
            None
        };

        Ok(Self { session, store })
    }
}
