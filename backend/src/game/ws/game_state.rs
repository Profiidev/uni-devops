use std::collections::HashMap;

use uuid::Uuid;

use crate::game::ws::{
  card::{Card, Hand},
  commands::{GameCommand, Raise},
  messages::{
    GameEnded, GameInfo, GameMessage, InvalidCommand, PlayerActed, PlayerEndedInfo, PlayerGameInfo,
    PlayerTurn, RoundEnded, YourCards,
  },
};

pub struct GameState {
  pub previous_player: Uuid,
  pub current_player: Uuid,
  pub small_blind: Uuid,
  pub revealed_cards: Vec<Card>,
  pub pot: i32,
  pub players: HashMap<Uuid, PlayerState>,
  pub deck: Vec<Card>,
  pub small_blind_amount: i32,
  pub players_checked: usize,
  pub game_running: bool,
  pub game_ended: bool,
}

pub struct PlayerState {
  pub id: Uuid,
  pub bet: i32,
  pub cash: i32,
  pub folded: bool,
  pub cards: [Card; 2],
}

impl GameState {
  pub fn init(players: Vec<Uuid>, small_blind_amount: i32, starting_cash: i32) -> Self {
    let small_blind = players[0];
    let big_blind = players[1];
    let current_player = *players.iter().cycle().nth(2).unwrap();

    let mut player_states = HashMap::new();
    let mut deck = Card::deck();

    for player in players {
      let cash = if player == small_blind {
        starting_cash - small_blind_amount
      } else if player == big_blind {
        starting_cash - (small_blind_amount * 2)
      } else {
        starting_cash
      };
      let bet = starting_cash - cash;

      player_states.insert(
        player,
        PlayerState {
          id: player,
          bet,
          cash,
          folded: false,
          cards: [deck.pop().unwrap(), deck.pop().unwrap()],
        },
      );
    }

    GameState {
      small_blind,
      previous_player: big_blind,
      current_player,
      revealed_cards: Vec::new(),
      pot: small_blind_amount * 3,
      players: player_states,
      deck,
      small_blind_amount,
      players_checked: 0,
      game_running: true,
      game_ended: false,
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
        .values()
        .map(|p| PlayerGameInfo {
          id: p.id,
          bet: p.bet,
          cash: p.cash,
          folded: p.folded,
        })
        .collect(),
    };

    GameMessage::GameInfo(game_info)
  }

  pub fn next_round(&mut self) -> Vec<GameMessage> {
    if self.game_running {
      return vec![GameMessage::InvalidCommand(InvalidCommand {
        message: "Game is still running".to_string(),
      })];
    }
    self.game_running = true;

    // Remove players with no cash left to continue
    self
      .players
      .retain(|_, player| player.cash > self.small_blind_amount * 2);

    let mut msgs = Vec::new();

    // Reset players' bets for the next round
    for player in self.players.values_mut() {
      player.bet = 0;
      player.folded = false;
    }

    // Reset pot and revealed cards
    self.pot = 0;
    self.revealed_cards.clear();
    self.players_checked = 0;

    // Shuffle the deck for the next round
    self.deck = Card::deck();

    let mut players_after_small_blind = self
      .players
      .keys()
      .copied()
      .cycle()
      .skip_while(|&p| p != self.small_blind)
      .skip(1);

    let small_blind = players_after_small_blind.next().unwrap();
    let big_blind = players_after_small_blind.next().unwrap();
    self.previous_player = big_blind;
    self.current_player = players_after_small_blind.next().unwrap();
    self.small_blind = small_blind;

    // Deal new cards to players
    for player in self.players.values_mut() {
      // Add blinds to the pot and deduct from players' cash
      if player.id == small_blind {
        player.cash -= self.small_blind_amount;
        player.bet = self.small_blind_amount;
        self.pot += self.small_blind_amount;
      } else if player.id == big_blind {
        player.cash -= self.small_blind_amount * 2;
        player.bet = self.small_blind_amount * 2;
        self.pot += self.small_blind_amount * 2;
      }

      player.cards = [self.deck.pop().unwrap(), self.deck.pop().unwrap()];
      msgs.push(GameMessage::YourCards(YourCards {
        cards: player.cards,
      }));
    }

    msgs.push(GameMessage::PlayerTurn(PlayerTurn {
      id: self.current_player,
    }));

    msgs
  }

  pub fn player_leaves(&mut self, player: Uuid) -> Vec<GameMessage> {
    if !self.players.contains_key(&player) {
      return vec![];
    }

    let mut msgs = Vec::new();

    // Remove the player from the game
    self.players.remove(&player);

    if self.current_player == player {
      // If the current player leaves, find the next player
      self.current_player = *self
        .players
        .keys()
        .cycle()
        .skip_while(|&&p| p != self.current_player)
        .skip(1)
        .find(|&&p| !self.players.get(&p).unwrap().folded)
        .unwrap();

      msgs.push(GameMessage::PlayerTurn(PlayerTurn {
        id: self.current_player,
      }));
    }

    msgs
  }

  pub fn handle_cmd(&mut self, cmd: GameCommand, player: Uuid) -> Vec<GameMessage> {
    if !self.game_running || self.game_ended {
      return vec![GameMessage::InvalidCommand(InvalidCommand {
        message: "Game is not running".to_string(),
      })];
    }

    if self.current_player != player {
      return vec![GameMessage::InvalidCommand(InvalidCommand {
        message: "It's not your turn".to_string(),
      })];
    }

    let previous = self.players.get(&self.previous_player).unwrap().bet;
    let current = self.players.get_mut(&self.current_player).unwrap();

    let mut msgs = Vec::new();
    match cmd {
      GameCommand::Call => {
        if previous > current.cash + current.bet {
          return vec![GameMessage::InvalidCommand(InvalidCommand {
            message: "You don't have enough cash to call".to_string(),
          })];
        }
        if previous == current.bet {
          return vec![GameMessage::InvalidCommand(InvalidCommand {
            message: "You must raise or check".to_string(),
          })];
        }
        current.cash -= previous - current.bet;
        current.bet = previous;
        self.pot += previous - current.bet;
        self.players_checked += 1;
        msgs.extend(self.check_next_round());
        self.previous_player = self.current_player;
      }
      GameCommand::Check => {
        if previous > current.bet {
          return vec![GameMessage::InvalidCommand(InvalidCommand {
            message: "You must call or raise".to_string(),
          })];
        }
        self.players_checked += 1;
        msgs.extend(self.check_next_round());
        self.previous_player = self.current_player;
      }
      GameCommand::Fold => {
        current.folded = true;
        if self.players.iter().filter(|(_, p)| !p.folded).count() == 1 {
          msgs.extend(self.end_game());
        }
      }
      GameCommand::Raise(Raise { amount }) => {
        let call_diff = previous - current.bet;
        if amount <= 0 {
          return vec![GameMessage::InvalidCommand(InvalidCommand {
            message: "Raise amount must be greater than 0".to_string(),
          })];
        }
        if amount + call_diff > current.cash {
          return vec![GameMessage::InvalidCommand(InvalidCommand {
            message: "You don't have enough cash to raise".to_string(),
          })];
        }
        current.cash -= amount + call_diff;
        current.bet += amount + call_diff;
        self.pot += amount + call_diff;
        self.previous_player = self.current_player;
        self.players_checked = 0;
      }
      _ => unreachable!("All other commands should be handled in the room"),
    }

    msgs.push(GameMessage::PlayerActed(PlayerActed {
      action: cmd.into(),
      id: self.current_player,
    }));

    self.current_player = *self
      .players
      .keys()
      .cycle()
      .skip_while(|&&p| p != self.current_player)
      .skip(1)
      .find(|&&p| !self.players.get(&p).unwrap().folded)
      .unwrap();

    if !self.game_ended && self.game_running {
      msgs.push(GameMessage::PlayerTurn(PlayerTurn {
        id: self.current_player,
      }));
    }

    msgs
  }

  fn check_next_round(&mut self) -> Vec<GameMessage> {
    if self.players_checked == self.players.iter().filter(|(_, p)| !p.folded).count() {
      if self.revealed_cards.len() < 5 {
        let mut msgs = Vec::new();
        if self.revealed_cards.is_empty() {
          // Flop
          self.revealed_cards.extend(self.deck.drain(0..3));
          for card in &self.revealed_cards {
            msgs.push(GameMessage::CardRevealed(*card));
          }
        } else if self.revealed_cards.len() == 3 {
          // Turn
          let card = self.deck.pop().unwrap();
          msgs.push(GameMessage::CardRevealed(card));
          self.revealed_cards.push(card);
        } else if self.revealed_cards.len() == 4 {
          // River
          let card = self.deck.pop().unwrap();
          msgs.push(GameMessage::CardRevealed(card));
          self.revealed_cards.push(card);
        }
        self.players_checked = 0;
        return msgs;
      } else {
        return self.end_game();
      }
    }

    vec![]
  }

  fn end_game(&mut self) -> Vec<GameMessage> {
    self.game_running = false;

    let remaining_players: Vec<_> = self.players.values().filter(|p| !p.folded).collect();
    let mut msgs = if remaining_players.len() != 1 {
      let community: [Card; 5] = self
        .revealed_cards
        .clone()
        .try_into()
        .expect("Should never fail");

      let hands = remaining_players
        .iter()
        .map(|p| (p.id, Hand::from_cards(p.cards, community)))
        .collect::<HashMap<_, _>>();

      let mut best_hand = None;
      let mut winners = Vec::new();
      for (id, hand) in &hands {
        if best_hand.is_none() || hand > best_hand.as_ref().unwrap() {
          best_hand = Some(hand.clone());
          winners.clear();
          winners.push(*id);
        } else if hand == best_hand.as_ref().unwrap() {
          winners.push(*id);
        }
      }

      let pot = self.pot / winners.len() as i32;

      for player in self.players.values_mut() {
        if winners.contains(&player.id) {
          player.cash += pot;
        }
      }

      vec![GameMessage::RoundEnded(RoundEnded {
        players: hands
          .iter()
          .map(|(&id, hand)| {
            let player = self.players.get(&id).unwrap();
            PlayerEndedInfo {
              id,
              cards: Some(player.cards),
              best_hand: Some(hand.clone()),
              winnings: if winners.contains(&id) { pot } else { 0 },
            }
          })
          .collect(),
      })]
    } else {
      vec![GameMessage::RoundEnded(RoundEnded {
        players: remaining_players
          .iter()
          .map(|p| PlayerEndedInfo {
            id: p.id,
            cards: Some(p.cards),
            best_hand: None,
            winnings: self.pot,
          })
          .collect(),
      })]
    };

    let non_broke_players: Vec<&PlayerState> =
      self.players.values().filter(|p| p.cash > 0).collect();
    if non_broke_players.len() == 1 {
      // If only one player has cash left, they win the game
      let winner = non_broke_players[0];
      msgs.push(GameMessage::GameEnded(GameEnded { winner: winner.id }));
      self.game_ended = true;
    }

    msgs
  }
}
