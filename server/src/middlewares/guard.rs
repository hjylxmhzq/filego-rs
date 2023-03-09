use std::collections::HashSet;

use actix_session::SessionExt;
use actix_web::dev::ServiceRequest;
use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::{error::AppError, session::SessionUtils, auth::ONETIME_TOKENS};

lazy_static! {
  pub static ref IGNORE_PATHS: Vec<Regex> = vec![Regex::new(r#"^/static/.+"#).unwrap()];
  pub static ref ALLOW_PATHS: HashSet<&'static str> = vec![
    "/auth/login",
    "/login",
    "/",
    // "/asset-manifest.json",
    // "/favicon.ico",
    // "/robots.txt"
  ]
  .into_iter()
  .collect();
}

pub fn guard(req: &ServiceRequest) -> Result<bool, AppError> {
  let (r, _) = req.parts();
  let query = qstring::QString::from(r.query_string());
  let one_time_token = query.get("one_time_token");
  if let Some(token) = one_time_token {
    let tokens = ONETIME_TOKENS.lock().unwrap();
    let exist = tokens.get(token);
    if let Some(token) = exist {
      if r.path().starts_with(&token.module_prefix) {
        return Ok(true);
      }
    }
  }
  let p = r.path();
  for re in IGNORE_PATHS.iter() {
    if re.is_match(p) {
      return Ok(true);
    }
  }
  if ALLOW_PATHS.contains(p) {
    return Ok(true);
  }
  let sess = r.get_session();
  sess.is_login()
}
