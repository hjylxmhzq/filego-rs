use clokwerk::{ScheduleHandle, Scheduler, TimeUnits};
use lazy_static::lazy_static;
use serde::Serialize;
use std::{
  collections::HashSet,
  path::PathBuf,
  sync::{Arc, Mutex, RwLock},
  thread::{self, sleep},
  time::{Duration, SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use crate::{
  db::SHARED_DB_CONN,
  models::{GalleryImage, NewGalleryImage},
  utils::error::AppError,
};

lazy_static! {
  pub static ref JOB_UPDATE_GALLERY: Arc<Mutex<UpdateGalleryJob>> =
    Arc::new(Mutex::new(UpdateGalleryJob::new()));
}
#[derive(Clone, Serialize, Debug)]
pub enum JobStatus {
  Running(u64),
  Idle,
}

pub struct UpdateGalleryJob {
  schedule_handle: Option<ScheduleHandle>,
  file_root: Option<PathBuf>,
  status: Arc<RwLock<JobStatus>>,
}

impl UpdateGalleryJob {
  pub fn new() -> Self {
    Self {
      schedule_handle: None,
      file_root: None,
      status: Arc::new(RwLock::new(JobStatus::Idle)),
    }
  }

  pub fn set_file_root(&mut self, file_root: &PathBuf) {
    self.file_root = Some(file_root.clone());
  }

  pub fn stop(&mut self) {
    if let Some(_) = self.schedule_handle {
      let s = self.schedule_handle.take().unwrap();
      s.stop();
    }
  }

  fn cleanup_db(updated_at_str: String) -> Result<(), AppError> {
    use crate::schema::gallery_images::dsl::*;
    use crate::schema::gallery_images::table;
    use diesel::prelude::*;
    let mut conn = SHARED_DB_CONN.lock().unwrap();
    let conn = &mut *conn;
    let _ = diesel::delete(table.filter(updated_at.is_not(updated_at_str)))
      .execute(conn)
      .unwrap();

    Ok(())
  }

  fn insert_images_into_db(images: Vec<String>, now: String) -> Result<(), AppError> {
    use crate::schema::gallery_images::dsl::*;
    use crate::schema::gallery_images::table;
    use diesel::prelude::*;

    let mut conn = SHARED_DB_CONN.lock().unwrap();
    let conn = &mut *conn;
    let exists = gallery_images
      .filter(file_path.eq_any(&images))
      .load::<GalleryImage>(conn)?;

    diesel::update(gallery_images)
      .filter(file_path.eq_any(&images))
      .set(updated_at.eq(now.clone()))
      .execute(conn)?;

    let set: HashSet<String> = exists.into_iter().map(|img| img.file_path).collect();
    let to_insert: Vec<NewGalleryImage> = images
      .into_iter()
      .filter(|img| {
        return !set.contains(img);
      })
      .map(|f| NewGalleryImage {
        file_path: f,
        size: 0,
        format: None,
        username: "".to_owned(),
        width: None,
        height: None,
        updated_at: now.clone(),
      })
      .collect();
    diesel::insert_into(table).values(to_insert).execute(conn)?;
    Ok(())
  }

  pub fn get_status(&self) -> JobStatus {
    let status = self.status.read().unwrap().clone();
    status
  }

  pub fn update_immediate(&self) {
    let status = self.status.clone();
    let file_root = self.file_root.clone();
    thread::spawn(move || {
      Self::update(status.clone(), file_root.as_ref().unwrap());
    });
  }

  fn update(status: Arc<RwLock<JobStatus>>, file_root: &PathBuf) {
    let mut status_lock = status.write().unwrap();
    match *status_lock {
      JobStatus::Idle => *status_lock = JobStatus::Running(0),
      JobStatus::Running(_) => return,
    };
    drop(status_lock);

    let file_root = file_root.clone();
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_millis()
      .to_string();
    let mut images = vec![];
    for entry in WalkDir::new(&file_root) {
      let entry = entry.unwrap();
      let dir = entry.path().strip_prefix(file_root.clone()).unwrap();
      let guess = mime_guess::from_path(dir);
      if let Some(mime) = guess.first() {
        if mime.to_string().contains("image") {
          images.push(dir.to_string_lossy().to_string());
          if images.len() > 25 {
            let len = images.len() as u64;
            let to_insert = images.drain(..).collect();
            Self::insert_images_into_db(to_insert, now.clone()).unwrap();
            let mut status_lock = status.write().unwrap();
            if let JobStatus::Running(sum) = *status_lock {
              *status_lock = JobStatus::Running(sum + len);
            }
            sleep(std::time::Duration::from_millis(100));
            drop(status_lock);
          }
        }
      }
    }
    if images.len() > 0 {
      let len = images.len() as u64;
      Self::insert_images_into_db(images, now.clone()).unwrap();
      let mut status_lock = status.write().unwrap();
      if let JobStatus::Running(sum) = *status_lock {
        *status_lock = JobStatus::Running(sum + len);
      }
    }
    Self::cleanup_db(now.clone()).unwrap();
    *status.write().unwrap() = JobStatus::Idle;
  }

  pub fn init(&mut self, internal: u32) -> Result<(), AppError> {
    self.stop();
    // Create a new scheduler
    let mut scheduler = Scheduler::new();
    // Add some tasks to it

    let file_root = self.file_root.clone().unwrap();

    let status = self.status.clone();
    let run = move || {
      Self::update(status.clone(), &file_root);
    };

    scheduler.every(internal.seconds()).run(run);

    // Or run it in a background thread
    let thread_handle = scheduler.watch_thread(Duration::from_millis(1000));
    // The scheduler stops when `thread_handle` is dropped, or `stop` is called
    self.schedule_handle = Some(thread_handle);
    Ok(())
  }
}
