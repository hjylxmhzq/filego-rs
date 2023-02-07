use crate::utils::error::AppError;
use actix_web::dev::Service;
use actix_web::{self, web, App, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  env, fs,
  net::SocketAddr,
  path::PathBuf,
  str::FromStr,
  sync::{Arc, RwLock},
};
use tokio::sync::Mutex;
use utils::auth::auto_create_user;
mod middlewares;
pub mod models;
mod routers;
pub mod schema;
mod utils;
use actix_web::middleware::Logger;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSessionData {
  username: String,
  is_login: bool,
  last_login: u64,
  user_root: String,
}

impl UserSessionData {
  pub fn new(username: &str, user_root: &str) -> UserSessionData {
    UserSessionData {
      is_login: true,
      username: username.to_string(),
      last_login: 0,
      user_root: user_root.to_string(),
    }
  }
}

#[allow(unused)]
#[derive(Debug)]
struct AppSession {
  users: HashMap<String, UserSessionData>,
}
#[allow(unused)]
pub struct AppState {
  config: AppConfig,
  session: AppSession,
  db: Mutex<SqliteConnection>,
}
#[derive(Debug)]
struct AppConfig {
  file_root: PathBuf,
  static_root: PathBuf,
  port: i32,
  host: String,
}

pub type AppData = Arc<RwLock<AppState>>;

#[allow(unused)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let mut app_state = init();
  let static_root = app_state.config.static_root.clone();
  let app_state = Arc::new(RwLock::new(app_state));

  let state = app_state.read().unwrap();
  let AppConfig { host, port, .. } = &state.config;

  let addr = SocketAddr::from_str(format!("{host}:{port}").as_str()).unwrap();
  drop(state);

  println!("server start on {addr:?}");

  println!("{static_root:?}");
  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(app_state.clone()))
      .service(actix_files::Files::new("/static", static_root.clone()))
      .service(routers::fs::file_routers())
      .service(routers::auth::auth_routers())
      .service(routers::index::index_routers())
      .wrap_fn(move |req, srv| {
        let s = srv.clone();
        let r = middlewares::guard::guard(&req);
        let mut resp = None;
        if let Ok(r) = r {
          if r {
            resp = Some(s.call(req));
          }
        }
        async move {
          if let Some(r) = resp {
            let r = r.await;
            return r;
          }
          let err: actix_web::Error = AppError::new("auth error").into();
          return Err(err);
        }
      })
      .wrap(middlewares::session::session())
      .wrap(Logger::default())
  })
  .bind(addr)?
  .run()
  .await
}

fn init() -> AppState {
  dotenv().map_or_else(
    |_| {
      println!("can not find .env file, use default value");
    },
    |v| {
      println!("find .env file at {v:?}");
    },
  );

  let port: i32 = std::env::var("PORT")
    .unwrap_or("7001".to_string())
    .parse()
    .unwrap();

  let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
  let file_root = std::env::var("FILE_ROOT").unwrap_or("files".to_string());
  let static_root = "static";
  let mut abs_file_root = env::current_dir().unwrap();
  let mut abs_static_root = env::current_dir().unwrap();
  abs_file_root.push(file_root);
  abs_static_root.push(static_root);
  fs::create_dir_all(&abs_file_root).unwrap();

  let mut conn = connect_db();

  auto_create_user(&mut conn);

  let state = AppState {
    config: AppConfig {
      file_root: abs_file_root,
      static_root: abs_static_root,
      port,
      host: host.clone(),
    },
    session: AppSession {
      users: HashMap::new(),
    },
    db: Mutex::new(conn),
  };

  state
}

fn connect_db() -> SqliteConnection {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let mut conn =
    SqliteConnection::establish(&database_url).expect("can not establish database connection");
  println!("database is connected");
  println!("running migrations");
  MigrationHarness::run_pending_migrations(&mut conn, MIGRATIONS).unwrap();
  println!("migrations finished");
  conn
}
