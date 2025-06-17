use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum GameCommand {
  RoomInfo,
  GameInfo,
  StartGame,
  Fold,
  Call,
  Raise(Raise),
  Check,
  BackToLobby,
}

#[derive(Deserialize)]
pub struct Raise {
  pub amount: i32,
}
