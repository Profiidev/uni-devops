use rocket::{
  Build, Rocket,
  fairing::{self, AdHoc},
};

pub trait RocketStateExt {
  fn manage_state(self) -> Self;
  fn manage_async_state(self) -> Self;
}

impl RocketStateExt for Rocket<Build> {
  fn manage_state(self) -> Self {
    //manage all sync state
    self
  }

  fn manage_async_state(self) -> Self {
    self.attach(AdHoc::try_on_ignite("Async state init", manage_async_state))
  }
}

async fn manage_async_state(server: Rocket<Build>) -> fairing::Result {
  //manage all async state
  Ok(server)
}
