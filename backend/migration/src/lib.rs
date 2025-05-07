pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![]
  }
}
mod m20250507_180655_create_key_table;
mod m20250507_180700_create_invalid_jwt_table;
