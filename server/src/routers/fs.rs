use std::borrow::Borrow;

use crate::utils::error::AppError;
use crate::utils::parser::parse_range;
use crate::utils::performance::timer;
use crate::utils::response::{create_binary_resp, create_stream_resp, EmptyResponseData};
use crate::utils::vfs::read_file_stream;
use crate::utils::{response::create_resp, vfs};
use crate::{AppData, any_params};
use actix_web::{web, HttpRequest, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetFilesOfDirReq {
  file: Option<String>,
}

#[derive(Serialize)]
pub struct GetFilesOfDirResp {
  files: Vec<String>,
}

pub async fn fs_actions(
  path: web::Path<(String,)>,
  req: web::Json<GetFilesOfDirReq>,
  query: web::Query<GetFilesOfDirReq>,
  req_raw: HttpRequest,
  state: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
  let file = any_params!(req, query, file).ok_or(AppError::new("param error"))?;
  let file_root = &state.read().unwrap().config.file_root;
  let action = path.into_inner().0;
  let headers = req_raw.headers();

  match action.as_str() {
    "read_dir" => {
      let files = vfs::read_dir(file_root.clone(), "".to_owned(), file.to_owned())
        .await
        .unwrap();

      let resp = GetFilesOfDirResp { files };

      Ok(create_resp(true, resp, ""))
    }

    "read" => {
      let file_stat = vfs::stat(file_root.clone(), "".to_owned(), file.to_owned()).await?;
      let range = parse_range(headers, file_stat.size)?;
      let stream = read_file_stream(
        file_root.clone(),
        "".to_owned(),
        file.to_owned(),
        Some(range),
      )
      .await?;
      let mime = mime_guess::from_path(file.to_owned())
        .first()
        .map(|m| m.to_string());
      Ok(create_stream_resp(stream, mime))
    }

    "delete" => {
      vfs::delete(file_root.clone(), "".to_owned(), file.to_owned()).await?;
      Ok(create_resp(true, EmptyResponseData::new(), "done"))
    }

    "stat" => {
      let file_stat = vfs::stat(file_root.clone(), "".to_owned(), file.to_owned()).await?;
      Ok(create_resp(true, file_stat, ""))
    }
    _ => Ok(create_resp(false, EmptyResponseData::new(), "error action")),
  }
}

pub async fn upload(
  mut parts: awmp::Parts,
  state: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
  let file_root = &state.read().unwrap().config.file_root;

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
  let dir = file_root.join(file_root).join(filename);
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

pub async fn read_image(
  req: web::Json<ReadImageReq>,
  query: web::Query<ReadImageReq>,
  state: web::Data<AppData>,
) -> Result<HttpResponse, AppError> {
  let file = any_params!(req, query, file).ok_or(AppError::new("error params"))?;

  let resize = any_params!(req, query, resize);
  let file_root = &state.read().unwrap().config.file_root;

  let mime = mime_guess::from_path(file.to_owned())
    .first()
    .map(|m| m.to_string());
  timer("read image");
  let img = vfs::read_image(file_root.clone(), "".to_string(), file.to_string(), resize).await?;
  timer("read image");
  Ok(create_binary_resp(img, mime))
}

pub fn file_routers() -> Scope {
  web::scope("/file")
    .route("/upload", web::post().to(upload))
    .route("/read_image", web::post().to(read_image))
    .route("/read_image", web::get().to(read_image))
    .route("/{action}", web::post().to(fs_actions))
    .route("/{action}", web::get().to(fs_actions))
}
