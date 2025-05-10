use entity::{player, prelude::*};
use sea_orm::{ActiveValue::Set, prelude::*};

pub struct PlayerTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> PlayerTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }
}
