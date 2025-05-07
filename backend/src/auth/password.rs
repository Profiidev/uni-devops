use rocket::{
  Route, State, get,
  http::{CookieJar, Status},
  post,
  serde::json::Json,
};
use rocket_okapi::{
  okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};
use schemars::JsonSchema;
use sea_orm_rocket::Connection;
use serde::{Deserialize, Serialize};

use crate::{
  db::{DB, DBTrait},
  error::{Error, Result},
};

use super::{
  jwt::{JwtClaims, JwtState, TokenRes},
  state::PasswordState,
  utils::hash_password,
};

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  openapi_get_routes_spec![settings: key, authenticate, change]
}

#[derive(Serialize, JsonSchema)]
struct KeyRes<'a> {
  key: &'a str,
}

/// # Key
///
/// Returns the public key used to encrypt the password when send to the server.
#[openapi(tag = "auth")]
#[get("/key")]
fn key(state: &State<PasswordState>) -> Json<KeyRes<'_>> {
  Json(KeyRes {
    key: &state.pub_key,
  })
}

#[derive(Deserialize, JsonSchema)]
struct LoginReq {
  email: String,
  password: String,
}

/// # Authenticate
///
/// Authenticate the user with email and password and set the JWT cookie.
/// The password is encrypted with the public key obtained from the `./key` endpoint.
#[openapi(tag = "auth", ignore = "conn")]
#[post("/authenticate", data = "<req>")]
async fn authenticate(
  req: Json<LoginReq>,
  state: &State<PasswordState>,
  jwt: &State<JwtState>,
  conn: Connection<'_, DB>,
  cookies: &CookieJar<'_>,
) -> Result<TokenRes> {
  let db = conn.into_inner();
  let user = db.tables().user().get_user_by_email(&req.email).await?;
  let hash = hash_password(state, &user.salt, &req.password)?;

  if hash != user.password {
    return Err(Error::Unauthorized);
  }

  let cookie = jwt.create_token(user.id)?;
  cookies.add(cookie);

  Ok(TokenRes)
}

#[derive(Deserialize, JsonSchema)]
struct PasswordChange {
  password: String,
  password_confirm: String,
}

/// # Change Password
///
/// Change the password of the user. The password and password confirmation
/// must be the same. The password is encrypted with the public key obtained
/// from the `./key` endpoint.
#[openapi(tag = "auth", ignore = "conn")]
#[post("/change", data = "<req>")]
async fn change(
  req: Json<PasswordChange>,
  auth: JwtClaims,
  state: &State<PasswordState>,
  conn: Connection<'_, DB>,
) -> Result<Status> {
  let db = conn.into_inner();
  let user = db.tables().user().get_user(auth.sub).await?;
  let hash = hash_password(state, &user.salt, &req.password)?;
  let hash_confirm = hash_password(state, &user.salt, &req.password_confirm)?;

  if hash != hash_confirm {
    return Err(Error::Conflict);
  }

  db.tables().user().change_password(user.id, hash).await?;

  Ok(Status::Ok)
}
