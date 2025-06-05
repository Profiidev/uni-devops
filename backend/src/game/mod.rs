use rocket::{Build, Rocket, Route};
use rocket_okapi::{
  get_nested_endpoints_and_docs, okapi::openapi3::OpenApi, settings::OpenApiSettings,
};
use state::RoomState;
use ws::state::GameState;

mod room;
mod state;
mod ws;

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  get_nested_endpoints_and_docs! {
    "/room" => room::routes(settings),
    "/ws" => ws::routes(settings),
  }
}

pub fn state(server: Rocket<Build>) -> Rocket<Build> {
  server.manage(RoomState::default()).manage(GameState::new())
}
