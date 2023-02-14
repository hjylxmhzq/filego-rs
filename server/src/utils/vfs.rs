use actix_web::body::SizedStream;
use actix_web::web::block;
use async_zip::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder};
use serde::Serialize;
use std::io::Cursor;
use std::path::Path;
use std::str::FromStr;
use std::time::UNIX_EPOCH;
use std::{fs::Metadata, io, path::PathBuf};
use tokio::fs::{self, File};
use tokio::io::{duplex, AsyncSeekExt, DuplexStream};
use tokio_util::io::ReaderStream;

use super::error::AppError;
use super::transform::ffmpeg_scale;

pub async fn read_dir(
  file_root: PathBuf,
  user_root: String,
  dir: String,
) -> Result<Vec<FileStatWithName>, AppError> {
  let odir = PathBuf::from(&dir);
  let dir = normailze_path(&file_root, &user_root, &dir);
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
    let buf = block(move || {
      let img = image::io::Reader::new(Cursor::new(&result))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
      let img = img.thumbnail(resize, resize);
      let mut buf = Vec::new();
      img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png);
      buf
    })
    .await?;
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

pub async fn read_video_transform_stream(
  file_root: PathBuf,
  user_root: String,
  file: String,
  resize: Option<u32>,
  bitrate: Option<u32>,
) -> Result<DuplexStream, AppError> {
  let dir = normailze_path(&file_root, &user_root, &file);
  let resize = resize.map_or(720, |v| v);
  let bitrate = bitrate.map_or(2000, |v| v);
  let stream = ffmpeg_scale(&dir, resize, bitrate).await;
  Ok(stream)
}

pub async fn create_dir(
  file_root: PathBuf,
  user_root: String,
  file: String,
) -> Result<(), AppError> {
  let dir = normailze_path(&file_root, &user_root, &file);
  let result = fs::create_dir(dir).await?;
  Ok(result)
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

pub async fn read_to_zip_stream(
  file_root: PathBuf,
  user_root: String,
  file: String,
) -> Result<ReaderStream<DuplexStream>, AppError> {
  let f = zip_path_to_stream(&file_root.join(user_root), &PathBuf::from_str(&file)?).await?;
  let reader = ReaderStream::new(f);
  Ok(reader)
}

pub fn ensure_parent_dir_sync(file: &PathBuf) -> Result<(), AppError> {
  let parent_dir = file.parent();
  if let Some(parent_dir) = parent_dir {
    std::fs::create_dir_all(parent_dir)?;
  }
  Ok(())
}

fn normailze_path(file_root: &PathBuf, user_root: &str, file: &str) -> PathBuf {
  let user_abs_root = file_root.join(user_root);
  let mut file = file;
  if file.starts_with("/") {
    file = &file[1..];
  }
  user_abs_root.join(file)
}

pub async fn zip_path_to_stream(
  base: &PathBuf,
  file: &PathBuf,
) -> Result<DuplexStream, std::io::Error> {
  #[async_recursion::async_recursion]
  async fn walk(
    base: &PathBuf,
    file: &PathBuf,
    writer: &mut ZipFileWriter<DuplexStream>,
  ) -> Result<(), std::io::Error> {
    let meta = tokio::fs::metadata(base.join(file)).await?;
    if meta.is_file() {
      let s = file.to_str().unwrap().to_string();
      #[cfg(debug_assertions)]
      println!("zip add entry: {s}");
      let entry = ZipEntryBuilder::new(s, Compression::Stored).build();
      let mut w = writer.write_entry_stream(entry).await.unwrap();
      let mut f = tokio::fs::File::open(base.join(file)).await?;
      tokio::io::copy(&mut f, &mut w).await?;
      w.close().await.unwrap();
    } else if meta.is_dir() {
      let mut files = tokio::fs::read_dir(base.join(file)).await?;
      while let Ok(Some(inner_file)) = files.next_entry().await {
        let filename = inner_file.file_name();
        walk(&base, &file.join(filename), writer).await?;
      }
    }
    Ok(())
  }

  let (w, r) = duplex(512 * 1024);

  let base = base.clone();
  let file = file.clone();
  tokio::spawn(async move {
    let mut writer = ZipFileWriter::new(w);
    walk(&base, &file, &mut writer).await.unwrap();
  });

  Ok(r)
}

pub fn ensure_dir_sync(dir: impl Into<PathBuf>) -> Result<(), AppError> {
  let p: PathBuf = dir.into();
  Ok(std::fs::create_dir_all(p)?)
}
