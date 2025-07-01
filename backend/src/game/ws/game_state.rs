use std::collections::HashMap;

use uuid::Uuid;

use crate::{
  db::tables::Tables,
  game::ws::{
    card::{Card, Hand},
    commands::{GameCommand, Raise},
    messages::{
      GameEnded, GameInfo, GameMessage, InvalidCommand, PlayerActed, PlayerAction, PlayerEndedInfo,
      PlayerGameInfo, PlayerTurn, RoundEnded, YourCards,
    },
  },
};

pub struct GameState {
  pub id: Uuid,
  pub previous_player: Uuid,
  pub current_player: Uuid,
  pub small_blind: Uuid,
  pub revealed_cards: Vec<Card>,
  pub pot: i32,
  pub players: Vec<PlayerState>,
  pub deck: Vec<Card>,
  pub small_blind_amount: i32,
  pub players_checked: usize,
  pub round_running: bool,
  pub game_ended: bool,
}

#[derive(Clone, Copy)]
pub struct PlayerState {
  pub id: Uuid,
  pub bet: i32,
  pub cash: i32,
  pub folded: bool,
  pub cards: [Card; 2],
}

impl GameState {
  pub fn init(id: Uuid, players: Vec<Uuid>, small_blind_amount: i32, starting_cash: i32) -> Self {
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
      let bet = starting_cash - cash;

      player_states.push(PlayerState {
        id: player,
        bet,
        cash,
        folded: false,
        cards: [deck.pop().unwrap(), deck.pop().unwrap()],
      });
    }

    GameState {
      id,
      small_blind,
      previous_player: big_blind,
      current_player,
      revealed_cards: Vec::new(),
      pot: small_blind_amount * 3,
      players: player_states,
      deck,
      small_blind_amount,
      players_checked: 0,
      round_running: true,
      game_ended: false,
    }
  }

  pub fn game_info(&self) -> GameInfo {
    GameInfo {
      current_player: self.current_player,
      small_blind: self.small_blind,
      revealed_cards: self.revealed_cards.clone(),
      pot: self.pot,
      round_running: self.round_running,
      players: self
        .players
        .iter()
        .map(|p| PlayerGameInfo {
          id: p.id,
          bet: p.bet,
          cash: p.cash,
          folded: p.folded,
        })
        .collect(),
    }
  }

  pub fn next_round(
    &mut self,
    active_players: Vec<Uuid>,
  ) -> (Vec<GameMessage>, HashMap<Uuid, GameMessage>) {
    if self.round_running {
      return (
        vec![GameMessage::InvalidCommand(InvalidCommand {
          message: "Game is still running".to_string(),
        })],
        HashMap::new(),
      );
    }
    self.round_running = true;

    // Remove players with no cash left to continue
    self
      .players
      .retain(|player| player.cash > self.small_blind_amount * 2);

    if self.players.len() < 2 {
      return (
        vec![GameMessage::InvalidCommand(InvalidCommand {
          message: "Not enough players to start a new round".to_string(),
        })],
        HashMap::new(),
      );
    }

    let mut msgs = Vec::new();

    // Reset players' bets for the next round
    for player in self.players.iter_mut() {
      player.bet = 0;
      if active_players.contains(&player.id) {
        player.folded = false;
      } else {
        player.folded = true; // If not active, they are considered folded
        msgs.push(GameMessage::PlayerActed(PlayerActed {
          id: player.id,
          action: PlayerAction::Fold,
        }));
      }
    }

    // Reset pot and revealed cards
    self.pot = 0;
    self.revealed_cards.clear();
    self.players_checked = 0;

    // Shuffle the deck for the next round
    self.deck = Card::deck();

    let mut players_after_small_blind = self
      .players
      .iter()
      .cycle()
      .skip_while(|&p| p.id != self.small_blind)
      .skip(1)
      .map(|p| p.id);

    let small_blind = players_after_small_blind.next().unwrap();
    let big_blind = players_after_small_blind.next().unwrap();
    self.previous_player = big_blind;
    self.current_player = players_after_small_blind.next().unwrap();
    self.small_blind = small_blind;

    // Deal new cards to players
    let mut player_cards: HashMap<Uuid, GameMessage> = HashMap::new();
    for player in self.players.iter_mut() {
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
      player_cards.insert(
        player.id,
        GameMessage::YourCards(YourCards {
          cards: player.cards,
        }),
      );
    }

    msgs.insert(0, GameMessage::GameStarted(self.game_info()));

    msgs.push(GameMessage::PlayerTurn(PlayerTurn {
      id: self.current_player,
    }));

    (msgs, player_cards)
  }

  pub async fn player_leaves(&mut self, player: Uuid, db: &Tables<'_>) -> Vec<GameMessage> {
    if !self.players.iter().any(|p| p.id == player) || !self.round_running {
      return vec![];
    }

    let mut msgs = Vec::new();

    // Fold player if they leave
    if let Some(player_state) = self.players.iter_mut().find(|p| p.id == player) {
      player_state.folded = true;

      msgs.push(GameMessage::PlayerActed(PlayerActed {
        id: player,
        action: PlayerAction::Fold,
      }));
    }

    if self.players.iter().filter(|p| !p.folded).count() == 1 {
      // If only one player remains, end the game
      msgs.extend(self.end_game(db).await);
      return msgs;
    }

    if self.current_player == player {
      // If the current player leaves, find the next player
      self.next_player();

      msgs.push(GameMessage::PlayerTurn(PlayerTurn {
        id: self.current_player,
      }));
    }

    msgs
  }

  pub async fn handle_cmd(
    &mut self,
    cmd: GameCommand,
    player: Uuid,
    db: &Tables<'_>,
  ) -> Vec<GameMessage> {
    if !self.round_running || self.game_ended {
      return vec![GameMessage::InvalidCommand(InvalidCommand {
        message: "Game is not running".to_string(),
      })];
    }

    if self.current_player != player {
      return vec![GameMessage::InvalidCommand(InvalidCommand {
        message: "It's not your turn".to_string(),
      })];
    }

    let previous = self
      .players
      .iter()
      .find(|p| p.id == self.previous_player)
      .unwrap()
      .bet;
    let current = self
      .players
      .iter_mut()
      .find(|p| p.id == self.current_player)
      .unwrap();

    let mut msgs = Vec::new();
    match cmd {
      GameCommand::Call => {
        if previous == current.bet {
          return vec![GameMessage::InvalidCommand(InvalidCommand {
            message: "You must raise or check".to_string(),
          })];
        }
        if current.cash <= 0 {
          return vec![GameMessage::InvalidCommand(InvalidCommand {
            message: "You have to check or fold, you have no cash left".to_string(),
          })];
        }
        let call_diff = (previous - current.bet).min(current.cash);
        current.cash -= call_diff;
        current.bet += call_diff;
        self.pot += call_diff;
        self.players_checked += 1;
        msgs.extend(self.check_next_round(db).await);
        self.previous_player = self.current_player;
      }
      GameCommand::Check => {
        if previous > current.bet && current.cash > 0 {
          return vec![GameMessage::InvalidCommand(InvalidCommand {
            message: "You must call or raise".to_string(),
          })];
        }
        self.players_checked += 1;
        msgs.extend(self.check_next_round(db).await);
        self.previous_player = self.current_player;
      }
      GameCommand::Fold => {
        current.folded = true;
        if self.players.iter().filter(|p| !p.folded).count() == 1 {
          msgs.extend(self.end_game(db).await);
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
        self.players_checked = 1;
      }
      _ => unreachable!("All other commands should be handled in the room"),
    }

    msgs.push(GameMessage::PlayerActed(PlayerActed {
      action: cmd.into(),
      id: self.current_player,
    }));

    self.next_player();

    if !self.game_ended && self.round_running {
      msgs.push(GameMessage::PlayerTurn(PlayerTurn {
        id: self.current_player,
      }));
    }

    msgs
  }

  fn next_player(&mut self) {
    self.current_player = self
      .players
      .iter()
      .cycle()
      .skip_while(|&p| p.id != self.current_player)
      .skip(1)
      .find(|&p| !p.folded)
      .map(|p| p.id)
      .unwrap();
  }

  async fn check_next_round(&mut self, db: &Tables<'_>) -> Vec<GameMessage> {
    if self.players_checked == self.players.iter().filter(|p| !p.folded).count() {
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
        return self.end_game(db).await;
      }
    }

    vec![]
  }

  async fn end_game(&mut self, db: &Tables<'_>) -> Vec<GameMessage> {
    self.round_running = false;

    let remaining_players: Vec<_> = self.players.iter().filter(|p| !p.folded).collect();
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

      let mut pot = self.pot;
      let mut winnings = HashMap::new();

      //first give every winner their bet back
      for player in self.players.iter_mut() {
        if winners.contains(&player.id) {
          player.cash += player.bet;
          pot -= player.bet;
          winnings.insert(player.id, player.bet);
        }
      }

      let mut winners_by_bet = self
        .players
        .iter()
        .filter(|p| winners.contains(&p.id))
        .copied()
        .collect::<Vec<_>>();
      winners_by_bet.sort_by_key(|p| p.bet);
      winners_by_bet.reverse();

      let winners_count = winners.len() as i32;
      let looser_count = self.players.len() as i32 - winners_count;

      if looser_count != 0 {
        while let Some(id) = winners_by_bet.pop().map(|p| p.id) {
          let player = self.players.iter_mut().find(|p| p.id == id).unwrap();

          let pot_per_player = pot / winners.len() as i32;
          let pot_per_player_from_each_looser = pot_per_player / looser_count;

          // If the player is all in, they can only win as much as they bet
          if player.cash == 0 && player.bet < pot_per_player_from_each_looser {
            player.cash += player.bet * looser_count;
            pot -= player.bet * looser_count;
            winnings
              .entry(id)
              .and_modify(|w| *w += player.bet * looser_count);
          } else {
            player.cash += pot_per_player;
            pot -= pot_per_player;
            winnings.entry(id).and_modify(|w| *w += pot_per_player);
          }
        }

        while pot > 0 {
          for player in self.players.iter_mut() {
            if !winners.contains(&player.id) {
              continue;
            }
            player.cash += 1;
            pot -= 1;
            winnings.entry(player.id).and_modify(|w| *w += 1);
            if pot <= 0 {
              break;
            }
          }
        }
      }

      vec![GameMessage::RoundEnded(RoundEnded {
        players: hands
          .iter()
          .map(|(&id, hand)| {
            let player = self.players.iter().find(|p| p.id == id).unwrap();
            PlayerEndedInfo {
              id,
              cards: Some(player.cards),
              best_hand: Some(hand.clone()),
              winnings: winnings.get(&id).copied().unwrap_or(0),
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

    let non_broke_players: Vec<&PlayerState> = self.players.iter().filter(|p| p.cash > 0).collect();
    if non_broke_players.len() == 1 {
      // If only one player has cash left, they win the game
      let winner = non_broke_players[0];
      msgs.push(GameMessage::GameEnded(GameEnded { winner: winner.id }));
      self.game_ended = true;
      db.room()
        .set_running(self.id, false)
        .await
        .expect("Failed to update room state");
    }

    msgs
  }
}
