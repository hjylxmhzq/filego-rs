use crate::utils::error::AppError;
use actix_web::dev::Service;
use actix_web::{self, web, App, HttpServer};
use chrono::NaiveTime;
use dotenv::dotenv;
use schedulers::update_file_index::JOB_UPDATE_GALLERY;
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
use tracing::log::{error, info};
use utils::auth::auto_create_user;
mod middlewares;
pub mod models;
mod routers;
mod schedulers;
pub mod schema;
mod utils;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

mod db;

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
  tracing_subscriber::fmt::init();

  let mut app_state = init();
  let static_root = app_state.config.static_root.clone();
  let app_state = Arc::new(RwLock::new(app_state));

  let state = app_state.read().unwrap();
  let AppConfig { host, port, .. } = &state.config;

  let addr = SocketAddr::from_str(format!("{host}:{port}").as_str()).unwrap();
  drop(state);

  info!("server start on {addr:?}");

  HttpServer::new(move || {
    let upload_temp_dir = std::env::var("UPLOAD_TEMPDIR").ok();
    let awmp_config;
    if let Some(upload_temp_dir) = upload_temp_dir {
      utils::vfs::ensure_dir_sync(&upload_temp_dir);
      awmp_config = awmp::PartsConfig::default().with_temp_dir(&upload_temp_dir);
    } else {
      awmp_config = awmp::PartsConfig::default();
    }
    App::new()
      .app_data(awmp_config)
      .app_data(web::Data::new(app_state.clone()))
      .service(routers::fs::file_routers())
      .service(routers::auth::auth_routers())
      .service(routers::gallery::gallery_routers())
      .service(routers::index::index_routers())
      .wrap(middlewares::static_server::static_server())
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
  })
  .bind(addr)?
  .run()
  .await
}

fn init() -> AppState {
  dotenv().map_or_else(
    |_| {
      error!("can not find .env file, use default value");
    },
    |v| {
      info!("find .env file at {v:?}");
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
  run_migrations(&mut conn);

  JOB_UPDATE_GALLERY
    .lock()
    .unwrap()
    .set_file_root(&abs_file_root);
  JOB_UPDATE_GALLERY
    .lock()
    .unwrap()
    .init(NaiveTime::from_hms_opt(3, 0, 0).unwrap())
    .unwrap();

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

pub fn run_migrations(conn: &mut SqliteConnection) {
  info!("database is connected");
  info!("running migrations");
  MigrationHarness::run_pending_migrations(conn, MIGRATIONS).unwrap();
  info!("migrations finished");
}

pub fn connect_db() -> SqliteConnection {
  let database_url = env::var("DATABASE_URL").unwrap_or("sqlite://./app.db".to_owned());
  let conn =
    SqliteConnection::establish(&database_url).expect("can not establish database connection");
  conn
}
