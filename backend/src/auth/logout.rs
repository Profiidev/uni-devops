use chrono::DateTime;
use rocket::{Route, State, get, http::CookieJar, post, serde::json::Json, time::Duration};
use rocket_okapi::{
  okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};
use sea_orm_rocket::Connection;

use crate::{
  db::{DB, DBTrait},
  error::Result,
};

use super::jwt::{JwtClaims, JwtInvalidState, JwtState, TokenRes};

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  openapi_get_routes_spec![settings: logout, test_token]
}

/// # Logout
///
/// This endpoint is used to log out a user by invalidating their JWT token and removing the token cookie.
#[openapi(tag = "auth", ignore = "conn")]
#[post("/logout")]
async fn logout(
  auth: JwtClaims,
  conn: Connection<'_, DB>,
  cookies: &CookieJar<'_>,
  state: &State<JwtInvalidState>,
  jwt: &State<JwtState>,
) -> Result<TokenRes> {
  let cookie = cookies.get("token").unwrap();

  let mut reset_cookie = jwt.create_cookie("".into(), true);
  reset_cookie.set_max_age(Duration::seconds(0));
  cookies.remove(reset_cookie);

  let db = conn.into_inner();
  let mut count = state.count.lock().await;
  db.tables()
    .invalid_jwt()
    .invalidate_jwt(
      cookie.value().to_string(),
      DateTime::from_timestamp(auth.exp, 0).unwrap(),
      &mut count,
    )
    .await?;
  Ok(TokenRes)
}

/// # Test Token
///
/// This endpoint is used to test if the JWT token is valid. It returns true if the token is valid, and false otherwise.
#[openapi(tag = "auth")]
#[get("/test_token")]
async fn test_token(
  auth: Option<JwtClaims>,
  cookies: &CookieJar<'_>,
  jwt: &State<JwtState>,
) -> Json<bool> {
  if auth.is_none() {
    let mut reset_cookie = jwt.create_cookie("".into(), true);
    reset_cookie.set_max_age(Duration::seconds(0));
    cookies.remove(reset_cookie);

    Json(false)
  } else {
    Json(true)
  }
}
