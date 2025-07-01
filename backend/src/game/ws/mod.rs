use std::time::Duration;

use rocket::{
  Route, State,
  futures::{SinkExt, StreamExt},
  get,
  serde::json,
  tokio::{self, time::sleep},
};
use rocket_okapi::{
  okapi::openapi3::OpenApi, openapi, openapi_get_spec, settings::OpenApiSettings,
};
use rocket_ws::{Channel, Message, WebSocket, result::Error};
use sea_orm_rocket::Connection;
use state::GlobalGameState;
use uuid::Uuid;

use crate::{
  auth::jwt::JwtClaims,
  db::{DB, DBTrait},
  error::Result,
  game::ws::messages::GameMessage,
};

mod card;
mod commands;
mod game_state;
mod messages;
pub mod state;

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  (rocket::routes![game], openapi_get_spec![settings: doc_game])
}

/// # WebSocket endpoint for game updates
///
/// This endpoint allows clients to connect to a game and send/receive updates
/// for the related game session.
#[openapi(tag = "Game")]
#[get("/<id>")]
#[allow(unused)]
async fn doc_game(id: Uuid, _auth: JwtClaims) -> Result<()> {
  let _ = id;
  Ok(())
}

/// The dummy function above is needed because the `#[openapi]` macro does not support lifetimes
#[get("/<id>")]
async fn game<'a>(
  auth: JwtClaims,
  ws: WebSocket,
  id: Uuid,
  state: &'a State<GlobalGameState>,
  conn: Connection<'a, DB>,
) -> Result<Channel<'a>> {
  log::info!("User {} connected to updater ws", auth.sub);
  let db = conn.into_inner();
  let user = db.tables().user().get_user(auth.sub).await?;
  let db = db.tables();

  let (mut recv, room) = state.join_room(id, user.name, auth.sub, &db).await?;

  Ok(ws.channel(move |mut stream| {
    Box::pin(async move {
      loop {
        tokio::select! {
          update = recv.recv() => {
            match update {
              Some(message) => {
                let message = json::to_string(&message).unwrap();
                let message = Message::Text(message);

                let _ = stream.send(message).await;
              }
              None => {
                let _ = stream.close(None).await;
                state.leave_room(id, auth.sub).await;
                break;
              }
            }
          }

          ws_msg = stream.next() => {
            match ws_msg {
              Some(Ok(Message::Close(_)) | Err(Error::AlreadyClosed | Error::ConnectionClosed)) | None => {
                state.leave_room(id, auth.sub).await;
                break;
              },
              Some(Ok(Message::Text(msg))) => {
                if let Err(err) = room.lock().await.handle_cmd(auth.sub, msg, &db).await {
                  log::warn!("Error handling command: {err}");
                }
              },
              _ => (),
            }
          }

          _ = sleep(Duration::from_secs(10)) => {
            // Heartbeat to keep the connection alive
            let heartbeat = Message::Text(json::to_string(&GameMessage::Heartbeat).unwrap());
            let _ = stream.send(heartbeat).await;
          }
        }
      }

      Ok(())
    })
  }))
}
