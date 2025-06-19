use entity::{prelude::*, room};
use sea_orm::{ActiveValue::Set, prelude::*};

pub struct RoomTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> RoomTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }

  pub async fn create(
    &self,
    owner: Uuid,
    starting_cash: i32,
    small_blind: i32,
  ) -> Result<room::Model, DbErr> {
    let room = room::ActiveModel {
      id: Set(Uuid::new_v4()),
      owner: Set(owner),
      created_at: Set(chrono::Utc::now().naive_utc()),
      game_running: Set(false),
      starting_cash: Set(starting_cash),
      small_blind: Set(small_blind),
    };

    room.insert(self.db).await
  }

  pub async fn delete(&self, id: Uuid, owner: Uuid) -> Result<(), DbErr> {
    room::Entity::delete_by_id(id)
      .filter(room::Column::Owner.eq(owner))
      .exec(self.db)
      .await?;
    Ok(())
  }

  pub async fn list(&self, owner: Uuid) -> Result<Vec<room::Model>, DbErr> {
    room::Entity::find()
      .filter(room::Column::Owner.eq(owner))
      .all(self.db)
      .await
  }

  pub async fn get(&self, id: Uuid) -> Result<room::Model, DbErr> {
    room::Entity::find_by_id(id)
      .one(self.db)
      .await?
      .ok_or(DbErr::RecordNotFound("Room not found".to_string()))
  }
}
