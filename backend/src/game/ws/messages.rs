use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum GameMessage {
  InvalidCommand(String),
  RoomInfo(RoomInfo),
}

#[derive(Serialize, Clone)]
pub struct RoomInfo {
  pub id: Uuid,
  pub players: Vec<PlayerInfo>,
}

#[derive(Serialize, Clone)]
pub struct PlayerInfo {
  pub id: Uuid,
  pub name: String,
}
