use serde::Deserialize;

use crate::game::ws::messages::PlayerAction;

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
  NextRound,
  BackToLobby,
}

#[derive(Deserialize)]
pub struct Raise {
  pub amount: i32,
}

impl From<GameCommand> for PlayerAction {
  fn from(command: GameCommand) -> Self {
    match command {
      GameCommand::Fold => PlayerAction::Fold,
      GameCommand::Call => PlayerAction::Call,
      GameCommand::Raise(Raise { amount }) => PlayerAction::Raise { amount },
      GameCommand::Check => PlayerAction::Check,
      _ => panic!("Invalid command for PlayerAction conversion"),
    }
  }
}
