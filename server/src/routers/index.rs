use actix_web::{web, Scope, HttpResponse};

use crate::{utils::error::AppError, middlewares::static_server::get_file};

pub fn index_routers() -> Scope {
  web::scope("")
    .route("/", web::get().to(index))
    .route("/login", web::get().to(index))
}

pub async fn index() -> Result<HttpResponse, AppError> {
  let content = get_file("index.html").ok_or(AppError::new("can not find index.html"))?;
  Ok(HttpResponse::Ok().body(content))
}
