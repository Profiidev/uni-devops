use invalid_jwt::InvalidJwtTable;
use key::KeyTable;
use room::RoomTable;
use sea_orm::DatabaseConnection;
use user::UserTable;

mod invalid_jwt;
mod key;
mod room;
mod user;

pub struct Tables<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> Tables<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }

  pub fn invalid_jwt(&self) -> InvalidJwtTable<'db> {
    InvalidJwtTable::new(self.db)
  }

  pub fn key(&self) -> KeyTable<'db> {
    KeyTable::new(self.db)
  }

  pub fn user(&self) -> UserTable<'db> {
    UserTable::new(self.db)
  }

  pub fn room(&self) -> RoomTable<'db> {
    RoomTable::new(self.db)
  }
}
