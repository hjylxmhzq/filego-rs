use actix_files::NamedFile;
use actix_web::{web, Scope};

use crate::{AppData, utils::error::AppError};


pub fn index_routers() -> Scope {
  web::scope("/").route("", web::get().to(index))
}

pub async fn index(state: web::Data<AppData>) -> Result<NamedFile, AppError> {
  let static_root = state.read().unwrap().config.static_root.clone();
  Ok(NamedFile::open(static_root.join("index.html"))?)
}
