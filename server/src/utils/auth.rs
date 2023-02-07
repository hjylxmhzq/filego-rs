use diesel::{RunQueryDsl, SqliteConnection};

use crate::{
  models::{NewUser, User},
  schema, utils::crypto::hash_pwd,
};
pub fn auto_create_user(db: &mut SqliteConnection) {
  use crate::schema::users::dsl::*;
  let user = users.first::<User>(db);
  if let Ok(_) = user {
    return ();
  }
  diesel::insert_into(schema::users::table)
    .values(NewUser {
      username: "admin",
      password: &hash_pwd("admin"),
      email: "",
      user_type: 0,
      user_root: "",
    })
    .execute(db)
    .unwrap();
  println!("create admin autmatically");
}
