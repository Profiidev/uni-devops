use std::io::Cursor;

use argon2::password_hash::SaltString;
use base64::prelude::*;
use image::{ImageFormat, imageops::FilterType};
use rocket::{Route, State, delete, get, post, serde::json::Json};
use rocket_okapi::{
  okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};
use rsa::rand_core::OsRng;
use schemars::JsonSchema;
use sea_orm_rocket::Connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  auth::{jwt::JwtClaims, state::PasswordState, utils::hash_password},
  db::{DB, DBTrait},
  error::Result,
};

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  openapi_get_routes_spec![settings: register, delete, update, update_image, info, info_by_id]
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
async fn register(
  req: Json<UserRegisterReq>,
  conn: Connection<'_, DB>,
  pw: &State<PasswordState>,
) -> Result<()> {
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

/// # Delete User
///
/// Deletes the user from the provided token.
#[openapi(tag = "user", ignore = "conn")]
#[delete("/delete")]
async fn delete(auth: JwtClaims, conn: Connection<'_, DB>) -> Result<()> {
  let db = conn.into_inner();

  db.tables().user().delete_user(auth.sub).await?;

  Ok(())
}

#[derive(Deserialize, JsonSchema)]
struct UserUpdateReq {
  /// The name of the user
  name: String,
}

/// # Update Profile
///
/// Updates the profile of the user.
#[openapi(tag = "user", ignore = "conn")]
#[post("/update", data = "<req>")]
async fn update(auth: JwtClaims, req: Json<UserUpdateReq>, conn: Connection<'_, DB>) -> Result<()> {
  let db = conn.into_inner();

  db.tables().user().update_user(auth.sub, req.0.name).await?;

  Ok(())
}

#[derive(Deserialize, JsonSchema)]
struct UserImageUpdateReq {
  /// The new image encoded in base64
  image: String,
}

/// # Update Image
///
/// Updates the image of the user.
#[openapi(tag = "user", ignore = "conn")]
#[post("/update_image", data = "<req>")]
async fn update_image(
  auth: JwtClaims,
  req: Json<UserImageUpdateReq>,
  conn: Connection<'_, DB>,
) -> Result<()> {
  let db = conn.into_inner();

  let bytes = BASE64_STANDARD.decode(req.0.image)?;
  let image = image::load_from_memory(&bytes)?;

  let scaled = image.resize_to_fill(256, 256, FilterType::Lanczos3);

  let mut cursor = Cursor::new(Vec::new());
  scaled.write_to(&mut cursor, ImageFormat::WebP)?;

  let cropped = BASE64_STANDARD.encode(cursor.into_inner());

  db.tables().user().update_image(auth.sub, cropped).await?;

  Ok(())
}

#[derive(Serialize, JsonSchema)]
struct UserInfoRes {
  /// The id of the user
  id: Uuid,
  /// The name of the user
  name: String,
  /// The email of the user
  email: String,
  /// The image of the user encoded in base64
  image: String,
}

/// # User Info
///
/// Returns the user info of the user.
#[openapi(tag = "user", ignore = "conn")]
#[get("/info")]
async fn info(auth: JwtClaims, conn: Connection<'_, DB>) -> Result<Json<UserInfoRes>> {
  let db = conn.into_inner();

  let user = db.tables().user().get_user(auth.sub).await?;

  Ok(Json(UserInfoRes {
    id: user.id,
    name: user.name,
    email: user.email,
    image: user.image,
  }))
}

/// # User Info by ID
///
/// Returns the user info of the user by id.
#[openapi(tag = "user", ignore = "conn")]
#[get("/info/<id>")]
async fn info_by_id(id: Uuid, conn: Connection<'_, DB>) -> Result<Json<UserInfoRes>> {
  let db = conn.into_inner();

  let user = db.tables().user().get_user(id).await?;

  Ok(Json(UserInfoRes {
    id: user.id,
    name: user.name,
    email: user.email,
    image: user.image,
  }))
}
