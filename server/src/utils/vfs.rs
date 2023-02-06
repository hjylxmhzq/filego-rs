use serde::Serialize;
use std::io::Cursor;
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::{fs::Metadata, io, path::PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncSeekExt;
use tokio_util::io::ReaderStream;

use super::error::AppError;

pub async fn read_dir(
  file_root: PathBuf,
  user_root: String,
  dir: String,
) -> Result<Vec<String>, io::Error> {
  let dir = file_root.join(user_root).join(dir);
  let mut result = fs::read_dir(dir).await?;
  let mut files_in_dir: Vec<String> = vec![];
  while let Result::Ok(Option::Some(dir_entry)) = result.next_entry().await {
    let filename = dir_entry.file_name().to_string_lossy().into_owned();
    files_in_dir.push(filename);
  }
  Ok(files_in_dir)
}
#[allow(unused)]
pub async fn read_image(
  file_root: PathBuf,
  user_root: String,
  file: String,
  resize: Option<u32>
) -> Result<Vec<u8>, AppError> {
  let dir = file_root.join(user_root).join(file);
  let result = fs::read(dir).await?;
  if let Some(resize) = resize {
    let img = image::io::Reader::new(Cursor::new(&result)).with_guessed_format()?.decode()?;
    let img = img.thumbnail(resize, resize);
    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png);
    return Ok(buf);
  }
  Ok(result)
}

pub async fn stat(
  file_root: PathBuf,
  user_root: String,
  file: String,
) -> Result<FileStat, AppError> {
  let dir = file_root.join(user_root).join(file);
  let meta = fs::metadata(dir).await?;
  convert_meta_to_struct(meta)
}
#[allow(unused)]
pub async fn create(
  file_root: PathBuf,
  user_root: String,
  file: String,
  buffer: Vec<u8>,
) -> Result<(), AppError> {
  let dir = file_root.join(user_root).join(&file);
  let parent = Path::new(&file).parent().unwrap_or(dir.as_path());
  fs::create_dir_all(parent).await?;
  Ok(fs::write(file, buffer).await?)
}

pub async fn delete(
  file_root: PathBuf,
  user_root: String,
  file: String,
) -> Result<(), AppError> {
  let dir = file_root.join(user_root.clone()).join(&file);
  let path_stat = stat(file_root, user_root.clone(), file).await?;
  if path_stat.is_dir {
    fs::remove_dir_all(dir).await?;
  } else {
    fs::remove_file(dir).await?;
  }
  Ok(())
}

#[derive(Serialize)]
pub struct FileStat {
  pub is_dir: bool,
  pub is_file: bool,
  pub file_type: String,
  pub size: u64,
  pub created: u128,
  pub modified: u128,
  pub accessed: u128,
}

pub fn convert_meta_to_struct(meta: Metadata) -> Result<FileStat, AppError> {
  Ok(FileStat {
    is_dir: meta.is_dir(),
    is_file: meta.is_file(),
    file_type: "".to_string(),
    size: meta.len(),
    created: meta.created()?.duration_since(UNIX_EPOCH)?.as_millis(),
    modified: meta.modified()?.duration_since(UNIX_EPOCH)?.as_millis(),
    accessed: meta.accessed()?.duration_since(UNIX_EPOCH)?.as_millis(),
  })
}

pub async fn read_file_stream(
  file_root: PathBuf,
  user_root: String,
  file: String,
  range: Option<(u64, u64)>,
) -> Result<ReaderStream<File>, AppError> {
  let dir = file_root.join(user_root).join(file);
  let mut f = tokio::fs::File::open(dir).await?;
  if let Some(seek_pos) = range {
    f.seek(io::SeekFrom::Start(seek_pos.0)).await.unwrap();
  }
  let reader = ReaderStream::new(f);
  Ok(reader)
}
