use rocket::{
  Build, Rocket,
  fairing::{self, AdHoc},
};

pub trait RocketStateExt {
  fn mount_routes(self) -> Self;
  fn init_state(self) -> Self;
  fn init_async_state(self) -> Self;
}

impl RocketStateExt for Rocket<Build> {
  fn mount_routes(self) -> Self {
    self.mount("/", crate::dummy::routes())
  }

  fn init_state(self) -> Self {
    //manage all sync state
    self
  }

  fn init_async_state(self) -> Self {
    self.attach(AdHoc::try_on_ignite("Async state init", manage_async_state))
  }
}

async fn manage_async_state(server: Rocket<Build>) -> fairing::Result {
  //manage all async state
  Ok(server)
}
