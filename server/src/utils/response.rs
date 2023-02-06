use actix_web::{HttpResponse, web};
use futures::Stream;
use serde::Serialize;

pub enum AppResponseStatus {
  Success = 0,
  Error = 1,
}
impl Serialize for AppResponseStatus {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: serde::Serializer {
      match &self {
          Self::Success => {
            serializer.serialize_i32(0)
          }
          Self::Error => {
            serializer.serialize_i32(1)
          }
      }
  }
}


#[derive(Serialize)]
pub struct AppResponse<T: Serialize> {
  status: AppResponseStatus,
  message: String,
  data: T,
}

#[derive(Serialize)]
pub struct EmptyResponseData {}
impl EmptyResponseData {
  pub fn new() -> EmptyResponseData {
    EmptyResponseData {}
  }
}

pub fn create_resp<T: Serialize>(success: bool, data: T, message: &str) -> HttpResponse {
  let resp = AppResponse {
    status: if success {
      AppResponseStatus::Success
    } else {
      AppResponseStatus::Error
    },
    message: message.to_string(),
    data,
  };
  let r = serde_json::to_string(&resp).map_or_else(
    |_| {
      HttpResponse::InternalServerError().content_type("application/json")
        .body(r#"{"status": 1, data: null, message: "internal server error"}"#)
    },
    |val| HttpResponse::Ok().content_type("application/json").body(val),
  );
  r
}
#[allow(unused)]
pub fn create_binary_resp(data: Vec<u8>, mime_type: Option<String>) -> HttpResponse {
  let mut resp = HttpResponse::Ok();
  resp.content_type(if let Some(mime) = mime_type { mime } else { "".to_owned() });
  resp.body(data)
}

pub fn create_stream_resp(stream: impl Stream<Item = Result<web::Bytes, std::io::Error>> + 'static, mime_type: Option<String>) -> HttpResponse {
  let mut resp = HttpResponse::Ok();
  resp.content_type(if let Some(mime) = mime_type { mime } else { "".to_owned() });
  resp.streaming(stream)
}