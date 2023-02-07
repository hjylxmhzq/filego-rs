use actix_web::body::SizedStream;
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
) -> Result<Vec<FileStatWithName>, AppError> {
  let odir = PathBuf::from(&dir);
  let dir = normailze_path(&file_root, &user_root, &dir);
  println!("{dir:?}");
  let mut result = fs::read_dir(&dir).await?;
  let mut files_in_dir: Vec<FileStatWithName> = vec![];
  while let Result::Ok(Option::Some(dir_entry)) = result.next_entry().await {
    let filename = dir_entry.file_name().to_string_lossy().into_owned();
    let file_path = odir.join(&filename);
    let file_path = file_path.to_str().map_or("", |v| v);
    let file_stat = stat(file_root.clone(), user_root.clone(), file_path).await?;
    let file_stat_with_name = FileStatWithName::new(&file_stat, &filename);
    files_in_dir.push(file_stat_with_name);
  }
  Ok(files_in_dir)
}
#[allow(unused)]
pub async fn read_image(
  file_root: PathBuf,
  user_root: String,
  file: String,
  resize: Option<u32>,
) -> Result<Vec<u8>, AppError> {
  let dir = normailze_path(&file_root, &user_root, &file);

  let result = fs::read(dir).await?;
  if let Some(resize) = resize {
    let img = image::io::Reader::new(Cursor::new(&result))
      .with_guessed_format()?
      .decode()?;
    let img = img.thumbnail(resize, resize);
    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png);
    return Ok(buf);
  }
  Ok(result)
}

pub async fn stat(file_root: PathBuf, user_root: String, file: &str) -> Result<FileStat, AppError> {
  let dir = normailze_path(&file_root, &user_root, &file);
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
  let dir = normailze_path(&file_root, &user_root, &file);
  let parent = Path::new(&file).parent().unwrap_or(dir.as_path());
  fs::create_dir_all(parent).await?;
  Ok(fs::write(file, buffer).await?)
}

pub async fn delete(file_root: PathBuf, user_root: String, file: String) -> Result<(), AppError> {
  let dir = normailze_path(&file_root, &user_root, &file);
  let path_stat = stat(file_root, user_root.clone(), &file).await?;
  if path_stat.is_dir {
    fs::remove_dir_all(dir).await?;
  } else {
    fs::remove_file(dir).await?;
  }
  Ok(())
}

#[derive(Serialize)]
#[mixin::declare]
pub struct FileStat {
  pub is_dir: bool,
  pub is_file: bool,
  pub file_type: String,
  pub size: u64,
  pub created: u128,
  pub modified: u128,
  pub accessed: u128,
}

#[mixin::insert(FileStat)]
#[derive(Serialize)]
pub struct FileStatWithName {
  pub name: String,
}

impl FileStatWithName {
  fn new(file_stat: &FileStat, name: &str) -> Self {
    let FileStat {
      is_dir,
      is_file,
      file_type,
      size,
      created,
      modified,
      accessed,
    } = file_stat;
    Self {
      name: name.to_string(),
      is_dir: *is_dir,
      is_file: *is_file,
      file_type: file_type.to_string(),
      size: *size,
      created: *created,
      modified: *modified,
      accessed: *accessed,
    }
  }
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
  range: (u64, u64),
) -> Result<SizedStream<ReaderStream<File>>, AppError> {
  let dir = normailze_path(&file_root, &user_root, &file);
  let mut f = tokio::fs::File::open(dir).await?;
  f.seek(io::SeekFrom::Start(range.0)).await.unwrap();
  let reader = ReaderStream::new(f);
  let reader = SizedStream::new(range.1 - range.0, reader);
  Ok(reader)
}

fn normailze_path(file_root: &PathBuf, user_root: &str, file: &str) -> PathBuf {
  let user_abs_root = file_root.join(user_root);
  let mut file = file;
  if file.starts_with("/") {
    file = &file[1..];
  }
  user_abs_root.join(file)
}
