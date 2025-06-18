use uuid::Uuid;

use crate::game::ws::{
  card::Card,
  commands::GameCommand,
  messages::{GameInfo, GameMessage, PlayerGameInfo},
};

pub struct GameState {
  pub current_player: Uuid,
  pub small_blind: Uuid,
  pub revealed_cards: Vec<Card>,
  pub pot: i32,
  pub players: Vec<PlayerState>,
  pub small_blind_amount: i32,
  pub starting_cash: i32,
  pub deck: Vec<Card>,
}

pub struct PlayerState {
  pub id: Uuid,
  pub cash: i32,
  pub folded: bool,
  pub cards: [Card; 2],
}

impl GameState {
  pub fn init(players: Vec<Uuid>, small_blind_amount: i32, starting_cash: i32) -> Self {
    let small_blind = players[0];
    let big_blind = players[1];
    let current_player = *players.iter().cycle().nth(2).unwrap();

    let mut player_states = Vec::new();
    let mut deck = Card::deck();

    for player in players {
      let cash = if player == small_blind {
        starting_cash - small_blind_amount
      } else if player == big_blind {
        starting_cash - (small_blind_amount * 2)
      } else {
        starting_cash
      };

      player_states.push(PlayerState {
        id: player,
        cash,
        folded: false,
        cards: [deck.pop().unwrap(), deck.pop().unwrap()],
      });
    }

    GameState {
      small_blind,
      current_player,
      revealed_cards: Vec::new(),
      pot: small_blind_amount * 3,
      players: player_states,
      small_blind_amount,
      starting_cash,
      deck,
    }
  }

  pub fn game_info(&self) -> GameMessage {
    let game_info = GameInfo {
      current_player: self.current_player,
      small_blind: self.small_blind,
      revealed_cards: self.revealed_cards.clone(),
      pot: self.pot,
      players: self
        .players
        .iter()
        .map(|p| PlayerGameInfo {
          id: p.id,
          cash: p.cash,
          folded: p.folded,
        })
        .collect(),
    };

    GameMessage::GameInfo(game_info)
  }

  pub fn handle_cmd(&mut self, cmd: GameCommand, player: Uuid) -> Option<GameMessage> {
    // player leaves and it is his turn
    match cmd {
      GameCommand::Call => (),
      GameCommand::Check => (),
      GameCommand::Fold => (),
      GameCommand::Raise(_) => (),
      _ => unreachable!("All other commands should be handled in the room"),
    }

    None
  }
}
