use invalid_jwt::InvalidJwtTable;
use key::KeyTable;
use sea_orm::DatabaseConnection;

mod invalid_jwt;
mod key;

pub struct Tables<'db> {
  #[allow(unused)]
  db: &'db DatabaseConnection,
}

impl<'db> Tables<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }

  pub fn invalid_jwt(self) -> InvalidJwtTable<'db> {
    InvalidJwtTable::new(self.db)
  }

  pub fn key(self) -> KeyTable<'db> {
    KeyTable::new(self.db)
  }
}
