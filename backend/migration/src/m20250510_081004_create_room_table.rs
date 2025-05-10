use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250507_191617_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Room::Table)
          .if_not_exists()
          .col(pk_uuid(Room::Id))
          .col(uuid(Room::Owner))
          .col(date_time(Room::CreatedAt))
          .col(boolean(Room::GameRunning))
          .col(integer(Room::StartingCash))
          .col(integer(Room::SmallBlind))
          .foreign_key(
            ForeignKey::create()
              .from(Room::Table, Room::Owner)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Room::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
pub enum Room {
  Table,
  Id,
  Owner,
  CreatedAt,
  GameRunning,
  StartingCash,
  SmallBlind,
}
