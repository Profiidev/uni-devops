use rocket::{get, serde::json::Json, Route, State};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings};
use schemars::JsonSchema;
use serde::Serialize;

use super::state::PasswordState;

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  openapi_get_routes_spec![settings: key]
}

#[derive(Serialize, JsonSchema)]
struct KeyRes<'a> {
  key: &'a str,
}

#[openapi(tag = "auth")]
#[get("/key")]
fn key(state: &State<PasswordState>) -> Json<KeyRes<'_>> {
  Json(KeyRes {
    key: &state.pub_key,
  })
}
