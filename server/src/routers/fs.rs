use crate::utils::error::AppError;
use crate::utils::parser::parse_range;
use crate::utils::response::{
  create_binary_resp, create_stream_resp, create_unsized_stream_resp, EmptyResponseData,
};
use crate::utils::session::SessionUtils;
use crate::utils::vfs::{
  ensure_parent_dir_sync, read_file_stream, read_to_zip_stream, FileStatWithName,
};
use crate::utils::{response::create_resp, vfs};
use crate::AppData;
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use tokio_util::io::ReaderStream;

#[derive(Deserialize)]
pub struct GetFilesOfDirReq {
  file: Option<String>,
}

#[derive(Serialize)]
pub struct GetFilesOfDirResp {
  files: Vec<FileStatWithName>,
}

pub async fn fs_actions_get(
  path: web::Path<(String,)>,
  query: web::Query<GetFilesOfDirReq>,
  req_raw: HttpRequest,
  state: web::Data<AppData>,
  sess: Session,
) -> Result<HttpResponse, AppError> {
  let file = query
    .borrow()
    .file
    .clone()
    .ok_or(AppError::new("query params error"))?;
  fs_actions(path, &file, true, req_raw, state, sess).await
}

pub async fn fs_actions_post(
  path: web::Path<(String,)>,
  query: web::Json<GetFilesOfDirReq>,
  req_raw: HttpRequest,
  state: web::Data<AppData>,
  sess: Session,
) -> Result<HttpResponse, AppError> {
  let file = query
    .borrow()
    .file
    .clone()
    .ok_or(AppError::new("query params error"))?;
  fs_actions(path, &file, false, req_raw, state, sess).await
}

pub async fn fs_actions(
  path: web::Path<(String,)>,
  file: &str,
  is_download: bool,
  req_raw: HttpRequest,
  state: web::Data<AppData>,
  sess: Session,
) -> Result<HttpResponse, AppError> {
  let file_root = &state.read().unwrap().config.file_root;
  let user_root = &sess.get_user_root()?;
  let action = path.into_inner().0;
  let headers = req_raw.headers();

  match action.as_str() {
    "read_dir" => {
      let files = vfs::read_dir(file_root.clone(), user_root.clone(), file.to_owned())
        .await
        .unwrap();

      let resp = GetFilesOfDirResp { files };

      Ok(create_resp(true, resp, ""))
    }

    "create_dir" => {
      vfs::create_dir(file_root.clone(), user_root.clone(), file.to_owned())
        .await
        .unwrap();

      Ok(create_resp(true, EmptyResponseData::new(), ""))
    }

    "read_compression" => {
      let stream =
        read_to_zip_stream(file_root.clone(), user_root.clone(), file.to_string()).await?;
      let resp = create_unsized_stream_resp(
        stream,
        Some("application/zip".to_string()),
        Some(&(file.to_string() + ".zip")),
      );
      Ok(resp)
    }

    "read" => {
      let file_stat = vfs::stat(file_root.clone(), user_root.clone(), file).await?;
      let (range_start, range_end, _) = parse_range(headers, file_stat.size)?;
      let stream = read_file_stream(
        file_root.clone(),
        user_root.to_owned(),
        file.to_owned(),
        (range_start, file_stat.size),
      )
      .await?;
      let mime = mime_guess::from_path(file.to_owned())
        .first()
        .map(|m| m.to_string());
      let resp = if is_download {
        create_stream_resp(
          stream,
          mime,
          Some(file),
          (range_start, range_end),
          range_end,
        )
      } else {
        create_stream_resp(
          stream,
          mime,
          None,
          (range_start, file_stat.size),
          file_stat.size,
        )
      };
      Ok(resp)
    }

    "delete" => {
      vfs::delete(file_root.clone(), user_root.clone(), file.to_owned()).await?;
      Ok(create_resp(true, EmptyResponseData::new(), "done"))
    }

    "stat" => {
      let file_stat = vfs::stat(file_root.clone(), user_root.clone(), file).await?;
      Ok(create_resp(true, file_stat, ""))
    }
    _ => Ok(create_resp(false, EmptyResponseData::new(), "error action")),
  }
}

pub async fn upload(
  parts: awmp::Parts,
  state: web::Data<AppData>,
  sess: Session,
) -> Result<HttpResponse, AppError> {
  let file_root = &state.read().unwrap().config.file_root;
  let user_root = &sess.get_user_root()?;

  let file_root = file_root.clone();
  let user_root = user_root.clone();
  web::block(move || -> Result<(), AppError> {
    let files = parts.files.into_inner();
    for (_, file) in files {
      if let Ok(file) = file {
        let filename = file.sanitized_file_name();
        let file_path = file_root.join(&user_root).join(filename);
        ensure_parent_dir_sync(&file_path)?;
        file.persist_at(file_path)?;
      }
    }
    Ok(())
  })
  .await??;

  Ok(create_resp(
    true,
    EmptyResponseData::new(),
    "upload file successfully",
  ))
}

#[derive(Deserialize)]
pub struct ReadImageReq {
  pub file: Option<String>,
  pub resize: Option<u32>,
}

#[derive(Deserialize)]
pub struct ReadVideoReq {
  pub file: Option<String>,
  pub resize: Option<u32>,
  pub bitrate: Option<u32>,
}

pub async fn read_image_get(
  query: web::Query<ReadImageReq>,
  state: web::Data<AppData>,
  sess: Session,
) -> Result<HttpResponse, AppError> {
  let file = query.file.clone().ok_or(AppError::new("params error"))?;
  let resize = query.resize;
  read_image(&file, resize, state, sess).await
}

pub async fn read_image_post(
  query: web::Json<ReadImageReq>,
  state: web::Data<AppData>,
  sess: Session,
) -> Result<HttpResponse, AppError> {
  let file = query.file.clone().ok_or(AppError::new("params error"))?;
  let resize = query.resize;
  read_image(&file, resize, state, sess).await
}

pub async fn read_image(
  file: &str,
  resize: Option<u32>,
  state: web::Data<AppData>,
  sess: Session,
) -> Result<HttpResponse, AppError> {
  let file_root = &state.read().unwrap().config.file_root;
  let user_root = sess.get_user_root()?;

  let mime = mime_guess::from_path(file.to_owned())
    .first()
    .map(|m| m.to_string());

  let img = vfs::read_image(
    file_root.clone(),
    user_root.clone(),
    file.to_string(),
    resize,
  )
  .await?;

  Ok(create_binary_resp(img, mime))
}

pub async fn read_video_transcode_get(
  query: web::Query<ReadVideoReq>,
  state: web::Data<AppData>,
  sess: Session,
) -> Result<HttpResponse, AppError> {
  let file = query.file.clone().ok_or(AppError::new("params error"))?;
  let resize = query.resize.clone();
  let bitrate = query.bitrate.clone();
  let file_root = &state.read().unwrap().config.file_root;
  let user_root = sess.get_user_root()?;

  let video_stream = vfs::read_video_transform_stream(
    file_root.clone(),
    user_root.clone(),
    file.to_string(),
    resize,
    bitrate,
  )
  .await?;

  let reader = ReaderStream::new(video_stream);
  Ok(create_unsized_stream_resp(
    reader,
    Some("video/mp4".to_string()),
    None,
  ))
}

pub fn file_routers() -> Scope {
  web::scope("/file")
    .route("/upload", web::post().to(upload))
    .route("/read_image", web::post().to(read_image_post))
    .route("/read_image", web::get().to(read_image_get))
    .route(
      "/read_video_transcode",
      web::get().to(read_video_transcode_get),
    )
    .route("/{action}", web::get().to(fs_actions_get))
    .route("/{action}", web::post().to(fs_actions_post))
}
