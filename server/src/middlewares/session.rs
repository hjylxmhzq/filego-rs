
use actix_session::{config::{CookieContentSecurity, PersistentSession}, storage, SessionMiddleware};
use actix_web::{cookie::{Key, time::Duration}, http::header::InvalidHeaderValue};

use crate::utils::error::AppError;

impl From<InvalidHeaderValue> for AppError {
  fn from(e: InvalidHeaderValue) -> Self {
    Self { msg: e.to_string() }
  }
}

pub fn session() -> SessionMiddleware<storage::CookieSessionStore> {
  let session_ttl = PersistentSession::default();
  let session_ttl = session_ttl.session_ttl(Duration::days(30));
  let store = storage::CookieSessionStore::default();
  SessionMiddleware::builder(store, Key::from(&[0; 64]))
    .cookie_secure(false)
    .cookie_content_security(CookieContentSecurity::Private).session_lifecycle(session_ttl)
    .build()
}
