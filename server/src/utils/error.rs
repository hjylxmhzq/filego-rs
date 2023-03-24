use std::{fmt::Display, time::SystemTimeError, convert::Infallible};

use actix_session::{SessionInsertError, SessionGetError};
use actix_web::{http::StatusCode, ResponseError, error::BlockingError};
use image::ImageError;

use super::response::{create_resp, EmptyResponseData};

#[derive(Debug)]
pub struct AppError {
  pub msg: String,
  pub status_code: StatusCode,
}

impl AppError {
  pub fn new(msg: &str) -> AppError {
    Self {
      msg: msg.to_string(),
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
  pub fn with_status(mut self, status: StatusCode) -> Self {
    self.status_code = status;
    self
  }
}

impl Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.msg)
  }
}

impl From<diesel::result::Error> for AppError {
  fn from(e: diesel::result::Error) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}

impl From<Infallible> for AppError {
  fn from(e: Infallible) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}

impl From<awmp::Error> for AppError {
  fn from(e: awmp::Error) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}

impl From<actix_web::error::HttpError> for AppError {
  fn from(e: actix_web::error::HttpError) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}

impl From<BlockingError> for AppError {
  fn from(e: BlockingError) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}

impl From<SystemTimeError> for AppError {
  fn from(e: SystemTimeError) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}

impl From<std::io::Error> for AppError {
  fn from(e: std::io::Error) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}

impl From<SessionGetError> for AppError {
  fn from(e: SessionGetError) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}

impl From<SessionInsertError> for AppError {
  fn from(e: SessionInsertError) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}


impl From<actix_web::Error> for AppError {
  fn from(e: actix_web::Error) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}

impl From<ImageError> for AppError {
  fn from(e: ImageError) -> Self {
    let msg = e.to_string();
    AppError::new(&msg)
  }
}

impl ResponseError for AppError {
  fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
    let mut resp = create_resp(false, EmptyResponseData::new(), &self.msg);
    let status = resp.status_mut();
    *status = self.status_code;
    resp
  }
  fn status_code(&self) -> actix_web::http::StatusCode {
    self.status_code
  }
}
