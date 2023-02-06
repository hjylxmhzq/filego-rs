use std::collections::HashSet;

use actix_session::SessionExt;
use actix_web::dev::ServiceRequest;
use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::{error::AppError, session::is_login};

lazy_static! {
  pub static ref IGNORE_PATHS: Vec<Regex> = vec![Regex::new(r#"^/static/.+"#).unwrap()];
  pub static ref ALLOW_PATHS: HashSet<&'static str> = vec!["/auth/login", "/"].into_iter().collect();
}

pub fn guard(req: &ServiceRequest) -> Result<bool, AppError> {
  let (r, _) = req.parts();
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
  is_login(sess)
}
