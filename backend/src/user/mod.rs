use argon2::password_hash::SaltString;
use rocket::{post, serde::json::Json, Route, State};
use rocket_okapi::{
  okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};
use rsa::rand_core::OsRng;
use schemars::JsonSchema;
use sea_orm_rocket::Connection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
  auth::{state::PasswordState, utils::hash_password}, db::{DBTrait, DB}, error::Result
};

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  openapi_get_routes_spec![settings: register]
}

#[derive(Deserialize, JsonSchema)]
struct UserRegisterReq {
  /// The email of the user
  email: String,
  /// The name of the user
  name: String,
  /// The password of the user. Encrypted with the public key
  password: String,
}

/// # Register User
///
/// Registers a new user.
#[openapi(tag = "user", ignore = "conn")]
#[post("/register", data = "<req>")]
async fn register(req: Json<UserRegisterReq>, conn: Connection<'_, DB>, pw: &State<PasswordState>,) -> Result<()> {
  let db = conn.into_inner();

  let salt = SaltString::generate(OsRng {}).to_string();
  let password = hash_password(pw, &salt, &req.password)?;

  let user = entity::user::Model {
    id: Uuid::new_v4(),
    name: req.0.name,
    email: req.0.email.to_lowercase(),
    password,
    image: String::new(),
    salt,
  };
  db.tables().user().create_user(user).await?;

  Ok(())
}
