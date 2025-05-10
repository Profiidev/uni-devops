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
      "/" => crate::dummy::routes(&openapi_settings),
    };

    rocket
  }

  fn init_state(self) -> Self {
    //manage all sync state
    self
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
  Ok(server)
}
