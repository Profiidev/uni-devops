use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum GameCommand {
  RoomInfo,
}
