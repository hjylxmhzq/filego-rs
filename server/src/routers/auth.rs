use actix_session::Session;
use diesel::prelude::*;
use std::borrow::Borrow;

use actix_web::{web, HttpResponse, Scope};
use serde::Deserialize;

use crate::{
  models::NewUser,
  models::User as TUser,
  utils::{
    crypto::hash_pwd,
    error::AppError,
    response::{create_resp, EmptyResponseData},
  },
  AppData, UserSessionData,
};

#[derive(Deserialize)]
pub struct User {
  name: String,
  password: String,
}

#[derive(Deserialize)]
pub struct RegisterUser {
  name: String,
  password: String,
  email: String,
}

pub async fn login(
  body: web::Json<User>,
  data: web::Data<AppData>,
  sess: Session,
) -> Result<HttpResponse, AppError> {
  use crate::schema::users::dsl::*;

  let name = &body.borrow().name;
  let pwd = &body.borrow().password;
  let hashed_pwd = hash_pwd(pwd);
  let state = data.borrow().write().unwrap();

  let mut db_mutex = state.db.lock().await;

  let db = &mut *db_mutex;

  let user = users
    .filter(username.eq(name).and(password.eq(hashed_pwd)))
    .load::<TUser>(db)?;

  drop(db_mutex);

  if user.len() == 0 {
    return Ok(create_resp(
      false,
      EmptyResponseData::new(),
      "password error or user not exists",
    ));
  }

  let user_data = sess.get::<UserSessionData>("user")?;

  match user_data {
    Some(mut user_data) => {
      user_data.is_login = true;
      sess.insert("user", user_data)?;
    }
    None => {
      let new_user_data = UserSessionData::new(name);
      sess.insert("user", new_user_data)?;
    }
  }

  Ok(create_resp(true, EmptyResponseData::new(), "done"))
}

pub async fn logout(sess: Session) -> Result<HttpResponse, AppError> {
  let user_data = sess.get::<UserSessionData>("user")?;

  match user_data {
    Some(mut user_data) => {
      user_data.is_login = false;
      sess.insert("user", user_data)?;
    }
    None => {
      return Err(AppError::new("user is not login"));
    }
  }

  Ok(create_resp(true, EmptyResponseData::new(), "done"))
}

pub async fn register(
  body: web::Json<RegisterUser>,
  data: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
  let name = &body.borrow().name;
  let pwd = &body.borrow().password;
  let hashed_pwd = hash_pwd(pwd);
  let email = &body.borrow().email;
  let state = data.borrow().write().unwrap();

  let mut conn = state.db.lock().await;
  let conn = &mut *conn;
  use crate::schema;

  diesel::insert_into(schema::users::table)
    .values(NewUser {
      username: name,
      password: &hashed_pwd,
      email,
    })
    .execute(conn)?;

  Ok(create_resp(true, EmptyResponseData::new(), "done"))
}

pub fn auth_routers() -> Scope {
  web::scope("/auth")
    .route("/login", web::post().to(login))
    .route("/register", web::post().to(register))
    .route("/logout", web::post().to(logout))
}
