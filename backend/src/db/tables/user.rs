use entity::{prelude::*, user};
use sea_orm::{ActiveValue::Set, prelude::*};
use uuid::Uuid;

pub struct UserTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> UserTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }

  pub async fn get_user(&self, id: Uuid) -> Result<user::Model, DbErr> {
    let user = User::find_by_id(id).one(self.db).await?;

    user.ok_or(DbErr::RecordNotFound("Not Found".into()))
  }

  pub async fn get_user_by_email(&self, email: &str) -> Result<user::Model, DbErr> {
    let user = User::find()
      .filter(user::Column::Email.eq(email.to_lowercase()))
      .one(self.db)
      .await?;

    user.ok_or(DbErr::RecordNotFound("Not Found".into()))
  }

  pub async fn change_password(&self, id: Uuid, password: String) -> Result<(), DbErr> {
    let mut user: user::ActiveModel = self.get_user(id).await?.into();

    user.password = Set(password);

    user.update(self.db).await?;

    Ok(())
  }
}
