use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum GameMessage {
  InvalidCommand(InvalidCommand),
  RoomInfo(RoomInfo),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InvalidCommand {
  pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RoomInfo {
  pub id: Uuid,
  pub players: Vec<PlayerInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
  pub id: Uuid,
  pub name: String,
}
