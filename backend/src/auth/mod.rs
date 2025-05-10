use jwt::{JwtInvalidState, JwtState};
use rocket::{Build, Rocket, Route};
use rocket_okapi::{
  get_nested_endpoints_and_docs, okapi::openapi3::OpenApi, settings::OpenApiSettings,
};
use sea_orm::DatabaseConnection;
use state::PasswordState;

pub mod jwt;
mod logout;
mod password;
pub mod state;
pub mod utils;

pub fn routes(settings: &OpenApiSettings) -> (Vec<Route>, OpenApi) {
  get_nested_endpoints_and_docs! {
    "/password" => password::routes(settings),
    "/" => logout::routes(settings),
  }
}

pub struct AsyncAuthStates {
  password: PasswordState,
  jwt: JwtState,
}

impl AsyncAuthStates {
  pub async fn new(db: &DatabaseConnection) -> Self {
    Self {
      password: PasswordState::init(db).await,
      jwt: JwtState::init(db).await,
    }
  }

  pub fn add(self, server: Rocket<Build>) -> Rocket<Build> {
    server.manage(self.password).manage(self.jwt)
  }
}

pub fn state(server: Rocket<Build>) -> Rocket<Build> {
  server.manage(JwtInvalidState::default())
}
