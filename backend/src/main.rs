use cors::cors;
use dotenv::dotenv;
use fern::Dispatch;
use rocket::{Config, launch};
use state::RocketStateExt;

mod cors;
mod dummy;
mod state;
mod error;

#[launch]
async fn rocket() -> _ {
  #[cfg(debug_assertions)]
  dotenv().ok();

  let level = std::env::var("RUST_LOG")
    .unwrap_or_else(|_| "info".to_string())
    .parse()
    .expect("Failed to parse RUST_LOG");
  Dispatch::new()
    .chain(Box::new(env_logger::builder().build()) as Box<dyn log::Log>)
    .level(level)
    .apply()
    .expect("Failed to initialize logger");

  let cors = cors();

  let figment = Config::figment()
    .merge(("address", "0.0.0.0"))
    .merge(("log_level", "normal"));

  rocket::custom(figment)
    .attach(cors)
    .mount_routes()
    .init_state()
    .init_async_state()
}
