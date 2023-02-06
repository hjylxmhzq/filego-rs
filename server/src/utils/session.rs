use actix_session::Session;

use crate::UserSessionData;

use super::error::AppError;

pub fn is_login(sess: Session) -> Result<bool, AppError> {
  let user_data = sess.get::<UserSessionData>("user")?;
  if let Some(user_data) = user_data {
    return Ok(user_data.is_login);
  }
  Ok(false)
}