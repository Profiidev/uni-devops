use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
  pub suit: Suit,
  pub rank: Rank,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suit {
  Hearts,
  Diamonds,
  Clubs,
  Spades,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rank {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

impl Card {
  pub fn deck() -> Vec<Card> {
    let mut deck = Vec::new();
    let suits = Suit::all();

    for suit in suits {
      for rank in Rank::all() {
        deck.push(Card { suit, rank });
      }
    }

    deck.shuffle(&mut rand::rng());

    deck
  }
}

impl Suit {
  fn all() -> Vec<Suit> {
    vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
  }
}

impl Rank {
  fn all() -> Vec<Rank> {
    vec![
      Rank::Two,
      Rank::Three,
      Rank::Four,
      Rank::Five,
      Rank::Six,
      Rank::Seven,
      Rank::Eight,
      Rank::Nine,
      Rank::Ten,
      Rank::Jack,
      Rank::Queen,
      Rank::King,
      Rank::Ace,
    ]
  }
}
