use std::sync::Arc;

use rocket::{
  Route, State,
  futures::{SinkExt, StreamExt},
  get,
  serde::json,
  tokio::{self, sync::Mutex},
};
use rocket_okapi::{
  okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
};
use rocket_ws::{Channel, Message, WebSocket, result::Error};
use state::GameState;
use uuid::Uuid;

use crate::auth::jwt::JwtClaims;

mod commands;
mod messages;
pub mod state;

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  openapi_get_routes_spec![settings: game]
}
/// # WebSocket endpoint for game updates
///
/// This endpoint allows clients to connect to a game and send/receive updates
/// for the related game session.
#[openapi(tag = "game")]
#[get("/<id>")]
async fn game(
  auth: JwtClaims,
  ws: WebSocket,
  id: Uuid,
  state: &State<Arc<Mutex<GameState>>>,
) -> Channel<'static> {
  log::info!("User {} connected to updater ws", auth.sub);
  let state = state.inner().clone();
  let (mut recv, room) = state.lock().await.join_room(id, auth.sub).await;

  ws.channel(move |mut stream| {
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
                state.lock().await.leave_room(id, auth.sub).await;
                break;
              }
            }
          }

          ws_msg = stream.next() => {
            match ws_msg {
              Some(Ok(Message::Close(_)) | Err(Error::AlreadyClosed | Error::ConnectionClosed)) | None => {
                state.lock().await.leave_room(id, auth.sub).await;
                break;
              },
              Some(Ok(Message::Text(msg))) => {
                room.lock().await.handle_cmd(auth.sub, msg).await;
              },
              _ => (),
            }
          }
        }
      }

      Ok(())
    })
  })
}
