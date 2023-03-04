use diesel::SqliteConnection;

use crate::models::GalleryImage;

use super::error::AppError;

pub fn get_all_images(db: &mut SqliteConnection, username_: &str) -> Result<Vec<GalleryImage>, AppError> {
  use crate::schema::gallery_images::dsl::*;
  use diesel::prelude::*;

  let exists = gallery_images.filter(username.is(username_)).load::<GalleryImage>(db)?;
  Ok(exists)
}
