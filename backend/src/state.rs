use rocket::{
  Build, Rocket, Route,
  fairing::{self, AdHoc},
};
use rocket_okapi::{
  mount_endpoints_and_merged_docs,
  okapi::openapi3::{OpenApi, Server},
  settings::OpenApiSettings,
  swagger_ui::{SwaggerUIConfig, make_swagger_ui},
};
use sea_orm_rocket::Database;

use crate::{
  auth::{self, AsyncAuthStates},
  db::DB,
};

pub trait RocketStateExt {
  fn mount_routes(self) -> Self;
  fn init_state(self) -> Self;
  fn init_async_state(self) -> Self;
}

impl RocketStateExt for Rocket<Build> {
  fn mount_routes(self) -> Self {
    let mut rocket = self.mount(
      "/docs",
      make_swagger_ui(&SwaggerUIConfig {
        url: "/backend/openapi.json".into(),
        ..Default::default()
      }),
    );

    let openapi_settings = OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
      rocket, "/", openapi_settings,
      "/" => openapi_base_url(),
      "/auth" => crate::auth::routes(&openapi_settings),
      "/user" => crate::user::routes(&openapi_settings),
    };

    rocket
  }

  fn init_state(self) -> Self {
    auth::state(self)
  }

  fn init_async_state(self) -> Self {
    self.attach(AdHoc::try_on_ignite("Async state init", manage_async_state))
  }
}

fn openapi_base_url() -> (Vec<Route>, OpenApi) {
  let mut openapi = OpenApi::new();
  openapi.servers = vec![Server {
    url: "/backend".into(),
    ..Default::default()
  }];
  (vec![], openapi)
}

async fn manage_async_state(server: Rocket<Build>) -> fairing::Result {
  //manage all async state
  let db = &DB::fetch(&server).unwrap().conn;

  let states = AsyncAuthStates::new(db).await;
  let server = states.add(server);

  Ok(server)
}
