use rocket::{Route, State, delete, get, post, serde::json::Json};
use rocket_okapi::{
  okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};
use schemars::JsonSchema;
use sea_orm_rocket::Connection;
use serde::Serialize;
use uuid::Uuid;

use crate::{
  auth::jwt::JwtClaims,
  db::{DB, DBTrait},
  error::{Error, Result},
};

use super::state::RoomState;

const MAX_ROOMS: usize = 10;

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  openapi_get_routes_spec![settings: create, delete, list]
}

#[derive(Serialize, JsonSchema)]
struct CreateRoomRes {
  /// The ID of the room.
  id: Uuid,
}

/// # Create room
///
/// Creates a new room with the current user as the owner.
#[openapi(tag = "game", ignore = "conn")]
#[post("/create")]
async fn create(
  auth: JwtClaims,
  conn: Connection<'_, DB>,
  state: &State<RoomState>,
) -> Result<Json<CreateRoomRes>> {
  let db = conn.into_inner();

  let count = db.tables().room().list(auth.sub).await?.len();

  if count >= MAX_ROOMS {
    return Err(Error::Conflict);
  }

  let room = db
    .tables()
    .room()
    .create(
      auth.sub,
      state.default_starting_cash,
      state.default_small_blind,
    )
    .await?;

  Ok(Json(CreateRoomRes { id: room.id }))
}

/// # Delete room
///
/// Deletes a room with the given ID if the current user is the owner.
#[openapi(tag = "game", ignore = "conn")]
#[delete("/<id>")]
async fn delete(id: Uuid, auth: JwtClaims, conn: Connection<'_, DB>) -> Result<()> {
  let db = conn.into_inner();
  db.tables().room().delete(id, auth.sub).await?;

  Ok(())
}

#[derive(Serialize, JsonSchema)]
struct RoomInfo {
  id: Uuid,
  owner: Uuid,
  created_at: chrono::NaiveDateTime,
  game_running: bool,
  starting_cash: i32,
  small_blind: i32,
}

impl From<entity::room::Model> for RoomInfo {
  fn from(value: entity::room::Model) -> Self {
    RoomInfo {
      id: value.id,
      owner: value.owner,
      created_at: value.created_at,
      game_running: value.game_running,
      starting_cash: value.starting_cash,
      small_blind: value.small_blind,
    }
  }
}

/// # List rooms
///
/// Lists all rooms owned by the current user.
#[openapi(tag = "game", ignore = "conn")]
#[get("/list")]
async fn list(auth: JwtClaims, conn: Connection<'_, DB>) -> Result<Json<Vec<RoomInfo>>> {
  let db = conn.into_inner();
  let rooms = db.tables().room().list(auth.sub).await?;

  Ok(Json(rooms.into_iter().map(RoomInfo::from).collect()))
}
