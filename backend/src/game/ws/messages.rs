use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game::ws::card::{Card, Hand};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum GameMessage {
  InvalidCommand(InvalidCommand),
  RoomInfo(RoomInfo),
  GameInfo(GameInfo),
  GameStarted(GameInfo),
  PlayerJoined(PlayerInfo),
  PlayerLeft(PlayerInfo),
  YourCards(YourCards),
  PlayerTurn(PlayerTurn),
  PlayerActed(PlayerActed),
  CardRevealed(Card),
  RoundEnded(RoundEnded),
  GameEnded(GameEnded),
  BackToLobby,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InvalidCommand {
  pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RoomInfo {
  pub id: Uuid,
  pub starting_cash: i32,
  pub small_blind: i32,
  pub owner: Uuid,
  pub game_running: bool,
  pub players: Vec<PlayerInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
  pub id: Uuid,
  pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerTurn {
  pub id: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct YourCards {
  pub cards: [Card; 2],
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerActed {
  pub id: Uuid,
  pub action: PlayerAction,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum PlayerAction {
  Fold,
  Call,
  Raise { amount: i32 },
  Check,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameInfo {
  pub revealed_cards: Vec<Card>,
  pub pot: i32,
  pub players: Vec<PlayerGameInfo>,
  pub current_player: Uuid,
  pub small_blind: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerGameInfo {
  pub id: Uuid,
  pub bet: i32,
  pub cash: i32,
  pub folded: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RoundEnded {
  pub players: Vec<PlayerEndedInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerEndedInfo {
  pub id: Uuid,
  pub cards: Option<[Card; 2]>,
  pub best_hand: Option<Hand>,
  pub winnings: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameEnded {
  pub winner: Uuid,
}
