use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250507_191617_create_user_table::User, m20250510_081004_create_room_table::Room};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Player::Table)
          .if_not_exists()
          .col(pk_uuid(Player::Id))
          .col(uuid(Player::Room))
          .col(uuid(Player::User))
          .col(integer(Player::Cash))
          .col(boolean(Player::Active))
          .foreign_key(
            ForeignKey::create()
              .from(Player::Table, Player::Room)
              .to(Room::Table, Room::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .from(Player::Table, Player::User)
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
      .drop_table(Table::drop().table(Player::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Player {
  Table,
  Id,
  Room,
  User,
  Cash,
  Active,
}
