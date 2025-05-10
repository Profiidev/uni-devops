use sea_orm::DatabaseConnection;

pub struct Tables<'db> {
  #[allow(unused)]
  db: &'db DatabaseConnection,
}

impl<'db> Tables<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }
}
