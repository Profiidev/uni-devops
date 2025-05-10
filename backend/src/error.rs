use rocket::{http::Status, response::Responder};
use rocket_okapi::{
  r#gen::OpenApiGenerator,
  okapi::{
    Map,
    openapi3::{RefOr, Response, Responses},
  },
  response::OpenApiResponderInner,
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
  #[allow(unused)]
  #[error("BadRequest")]
  BadRequest,
  #[error("Unauthorized")]
  Unauthorized,
  #[allow(clippy::enum_variant_names)]
  #[allow(unused)]
  #[error("InternalServerError")]
  InternalServerError,
  #[error("Conflict")]
  Conflict,
  #[allow(unused)]
  #[error("Gone")]
  Gone,
  #[error("Uuid Error {source:?}")]
  Uuid {
    #[from]
    source: uuid::Error,
  },
  #[error("Io Error {source:?}")]
  IO {
    #[from]
    source: std::io::Error,
  },
  #[error("DB error {source:?}")]
  DB {
    #[from]
    source: sea_orm::DbErr,
  },
  #[error("Argon2 Error {source:?}")]
  Argon2 {
    #[from]
    source: argon2::password_hash::Error,
  },
  #[error("Base64 Error {source:?}")]
  Base64 {
    #[from]
    source: base64::DecodeError,
  },
  #[error("Rsa error {source:?}")]
  Rsa {
    #[from]
    source: rsa::errors::Error,
  },
  #[error("Jwt Error {source:?}")]
  Jwt {
    #[from]
    source: jsonwebtoken::errors::Error,
  },
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
  fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
    log::error!("{:?}", &self);
    match self {
      Self::Unauthorized => Status::Unauthorized.respond_to(request),
      Self::Gone => Status::Gone.respond_to(request),
      Self::InternalServerError => Status::InternalServerError.respond_to(request),
      Self::Conflict => Status::Conflict.respond_to(request),
      _ => Status::BadRequest.respond_to(request),
    }
  }
}

impl OpenApiResponderInner for Error {
  fn responses(_: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
    let mut responses = Map::new();

    responses.insert(
      Status::Unauthorized.code.to_string(),
      RefOr::Object(Response {
        description:
          "[Unauthorized](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401). The request requires user authentication. The client should authenticate itself to get the requested response."
            .to_string(),
        ..Default::default()
      }),
    );

    responses.insert(
      Status::Gone.code.to_string(),
      RefOr::Object(Response {
        description:
          "[Gone](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/410). The resource requested is no longer available on the server and no forwarding address is known. A client should not repeat the request without modifications."
            .to_string(),
        ..Default::default()
      }),
    );

    responses.insert(
      Status::InternalServerError.code.to_string(),
      RefOr::Object(Response {
        description:
          "[Internal Server Error](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500). An internal server error occurred. The server encountered an unexpected condition that prevented it from fulfilling the request."
            .to_string(),
        ..Default::default()
      }),
    );

    responses.insert(
      Status::Conflict.code.to_string(),
      RefOr::Object(Response {
        description:
          "[Conflict](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/409). The request could not be completed due to a conflict with the current state of the target resource."
            .to_string(),
        ..Default::default()
      }),
    );

    responses.insert(
      Status::BadRequest.code.to_string(),
      RefOr::Object(Response {
        description:
          "[Bad Request](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400). A client side error occurred. The request was malformed or invalid."
            .to_string(),
        ..Default::default()
      }),
    );

    Ok(Responses {
      responses,
      ..Default::default()
    })
  }
}
