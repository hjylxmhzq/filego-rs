use actix_session::{config::CookieContentSecurity, storage, SessionMiddleware};
use actix_web::{cookie::Key, http::header::InvalidHeaderValue};

use crate::utils::error::AppError;

impl From<InvalidHeaderValue> for AppError {
  fn from(e: InvalidHeaderValue) -> Self {
    Self { msg: e.to_string() }
  }
}

pub fn session() -> SessionMiddleware<storage::CookieSessionStore> {
  let store = storage::CookieSessionStore::default();
  SessionMiddleware::builder(store, Key::from(&[0; 64]))
    .cookie_secure(false)
    .cookie_content_security(CookieContentSecurity::Private)
    .build()
}
