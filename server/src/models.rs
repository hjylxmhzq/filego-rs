use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;


#[derive(Queryable)]
#[diesel(table_name = users)]
pub struct User {
  pub username: String,
  pub password: String,
  pub email: String,
  pub user_type: i32,
  pub user_root: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub password: &'a str,
  pub email: &'a str,
  pub user_type: i32,
  pub user_root: &'a str,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = gallery_images)]
pub struct NewGalleryImage {
  pub file_path: String,
  pub username: String,
  pub size: i32,
  pub width: Option<i32>,
  pub height: Option<i32>,
  pub format: Option<String>,
  pub updated_at: String,
}

#[derive(Queryable, Debug, Serialize)]
#[diesel(table_name = gallery_images)]
pub struct GalleryImage {
  pub file_path: String,
  pub username: String,
  pub size: i32,
  pub width: Option<i32>,
  pub height: Option<i32>,
  pub format: Option<String>,
  pub updated_at: String,
}
