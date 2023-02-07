use crate::schema::*;
use diesel::prelude::*;


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
