
use actix_session::Session;

use crate::UserSessionData;

use super::error::AppError;

pub fn is_login(sess: &Session) -> Result<bool, AppError> {
  let user_data = sess.get::<UserSessionData>("user")?;
  if let Some(user_data) = user_data {
    return Ok(user_data.is_login);
  }
  Ok(false)
}

pub fn get_user_data(sess: &Session) -> Result<UserSessionData, AppError> {
  let user_data = sess
    .get::<UserSessionData>("user")?
    .ok_or(AppError::new("no user session data"))?;

  return Ok(user_data);
}

pub trait SessionUtils {
  fn is_login(&self) -> Result<bool, AppError>;
  fn get_user_data(&self) -> Result<UserSessionData, AppError>;
  fn get_user_root(&self) -> Result<String, AppError>;
}

impl SessionUtils for Session {
  fn is_login(&self) -> Result<bool, AppError> {
      is_login(self)
  }
  fn get_user_data(&self) -> Result<UserSessionData, AppError> {
      get_user_data(self)
  }
  fn get_user_root(&self) -> Result<String, AppError> {
    let data = self.get_user_data()?;
    Ok(data.user_root)
  }
}