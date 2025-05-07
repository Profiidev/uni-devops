use rocket::{Route, get};
use rocket_okapi::{
  okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  openapi_get_routes_spec![settings: test]
}

/// # Dummy
/// 
/// A dummy endpoint to test the backend
#[openapi(tag = "dummy")]
#[get("/test")]
fn test() -> String {
  "Test".into()
}
