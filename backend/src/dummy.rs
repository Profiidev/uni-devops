use rocket::{get, post, serde::json::Json, Route};
use rocket_okapi::{
  okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

use crate::error::Result;

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  openapi_get_routes_spec![settings: test, test_2]
}

/// # Dummy
///
/// A dummy endpoint to test the backend
#[openapi(tag = "dummy")]
#[get("/test")]
fn test() -> String {
  "Test".into()
}

/// # Dummy with error
///
/// A dummy endpoint that could fail
#[openapi(tag = "dummy")]
#[post("/test_2", data = "<req>")]
fn test_2(req: Json<String>) -> Result<String> {
  Ok(req.into_inner())
}
