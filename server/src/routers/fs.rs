use std::borrow::Borrow;
use crate::utils::error::AppError;
use crate::utils::parser::parse_range;
use crate::utils::response::{create_binary_resp, create_stream_resp, EmptyResponseData, create_unsized_stream_resp};
use crate::utils::session::SessionUtils;
use crate::utils::vfs::{read_file_stream, FileStatWithName, read_to_zip_stream};
use crate::utils::{response::create_resp, vfs};
use crate::AppData;
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Scope};
use serde::{Deserialize, Serialize};


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

    "read_compression" => {
      let stream = read_to_zip_stream(file_root.clone(), user_root.clone(), file.to_string()).await?;
      let resp = create_unsized_stream_resp(stream, Some("application/zip".to_string()), Some(&(file.to_string() + ".zip")));
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
        create_stream_resp(stream, mime, None, (range_start, file_stat.size), file_stat.size)
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
  mut parts: awmp::Parts,
  state: web::Data<AppData>,
  sess: Session,
) -> Result<HttpResponse, AppError> {
  let file_root = &state.read().unwrap().config.file_root;
  let user_root = &sess.get_user_root()?;

  let query = parts.texts.as_pairs();
  let mut filename = String::new();
  for n in query {
    if n.0 == "filename" {
      filename = n.1.to_string();
    }
  }
  if filename.is_empty() {
    return Err(AppError::new("can not find filename in formdata"));
  }
  let dir = file_root.join(user_root).join(filename);
  web::block(move || -> Result<(), AppError> {
    let f = parts.files.take("file").pop().ok_or(AppError::new(""))?;
    f.persist_at(dir)?;
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

pub fn file_routers() -> Scope {
  web::scope("/file")
    .route("/upload", web::post().to(upload))
    .route("/read_image", web::post().to(read_image_post))
    .route("/read_image", web::get().to(read_image_get))
    .route("/{action}", web::get().to(fs_actions_get))
    .route("/{action}", web::post().to(fs_actions_post))
}
