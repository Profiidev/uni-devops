use rocket::{Route, get};

pub fn routes() -> Vec<Route> {
  rocket::routes![test]
}

#[get("/test")]
fn test() -> String {
  "Test".into()
}
