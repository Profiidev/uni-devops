use std::{cmp::Ordering, collections::HashMap};

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
  pub suit: Suit,
  pub rank: Rank,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
  Hearts,
  Diamonds,
  Clubs,
  Spades,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl PartialOrd for Suit {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Suit {
  fn cmp(&self, _other: &Self) -> Ordering {
    Ordering::Equal // Suits are not ordered in poker, so we return Equal
  }
}

/// Ordering of the variants and the fields is VERY important for determining the strength of the hand.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type")]
pub enum Hand {
  HighCard {
    cards: [Rank; 5], // the five highest cards in the hand, sorted by rank
  },
  OnePair {
    rank: Rank,
    kickers: [Rank; 3], // the three remaining cards that are not part of the pair
  },
  TwoPair {
    high_pair: Rank,
    low_pair: Rank,
    kicker: Rank, // the fifth card that is not part of the two pairs
  },
  ThreeOfAKind {
    rank: Rank,
    kickers: [Rank; 2], // the two remaining cards that are not part of the three of a kind
  },
  Straight {
    rank: Rank, // the highest card in the straight
  },
  Flush {
    suit: Suit, // the suit of the flush
  },
  FullHouse {
    triplet: Rank,
    pair: Rank,
  },
  FourOfAKind {
    rank: Rank,
    kicker: Rank, // the fifth card that is not part of the four of a kind
  },
  StraightFlush {
    rank: Rank, // the highest card in the straight flush
    suit: Suit, // the suit of the straight flush
  },
  RoyalFlush {
    suit: Suit, // the suit of the royal flush
  },
}

impl Hand {
  pub fn from_cards(player: [Card; 2], community: [Card; 5]) -> Hand {
    // all 7 cards combined
    let mut all_cards = player.to_vec();
    all_cards.extend_from_slice(&community);

    // Sort the cards by rank
    all_cards.sort_by(|a, b| a.rank.cmp(&b.rank));
    all_cards.reverse(); // Highest rank first
    // get count of each rank
    let mut rank_count = HashMap::new();
    for card in &all_cards {
      *rank_count.entry(card.rank).or_insert(0) += 1;
    }
    let mut rank_sorted_by_count: Vec<_> = rank_count
      .iter()
      .map(|(&rank, &count)| (rank, count))
      .collect();
    rank_sorted_by_count.sort_by(|a, b| match b.1.cmp(&a.1) {
      Ordering::Equal => a.0.cmp(&b.0),
      other => other,
    });

    // Check for flush
    let mut suits = HashMap::new();
    for card in &all_cards {
      *suits.entry(card.suit).or_insert(0) += 1;
    }
    let is_flush = suits.iter().find(|&(_, count)| *count >= 5);

    // Check for straight
    let mut all_cards_deduped: Vec<_> = all_cards.clone();
    all_cards_deduped.dedup();

    let mut straight_rank = None;
    if all_cards_deduped.len() >= 5 {
      for i in 0..all_cards_deduped.len() - 4 {
        let window = &all_cards_deduped[i..i + 5];
        if window
          .windows(2)
          .all(|w| w[1].rank as u8 == w[0].rank as u8 + 1)
        {
          if let Some(rank) = straight_rank {
            if window[4].rank < rank {
              continue; // We want the highest straight
            }
          }
          straight_rank = Some(window[4].rank);
        }
      }
    }

    // Determine the hand type
    if is_flush.is_some() && straight_rank.is_some() {
      // ! if let chains will be stabilized in rust 1.88 we have 1.87 :-)
      let straight_rank = straight_rank.unwrap();
      let suit = *is_flush.unwrap().0;
      if straight_rank == Rank::Ace {
        Hand::RoyalFlush { suit }
      } else {
        Hand::StraightFlush {
          rank: straight_rank,
          suit,
        }
      }
    } else if rank_sorted_by_count[0].1 == 4 {
      Hand::FourOfAKind {
        rank: rank_sorted_by_count[0].0,
        kicker: rank_sorted_by_count[1].0,
      }
    } else if rank_sorted_by_count[0].1 == 3 && rank_sorted_by_count[1].1 >= 2 {
      Hand::FullHouse {
        triplet: rank_sorted_by_count[0].0,
        pair: rank_sorted_by_count[1].0,
      }
    } else if let Some((&suit, _)) = is_flush {
      Hand::Flush { suit }
    } else if let Some(straight_rank) = straight_rank {
      Hand::Straight {
        rank: straight_rank,
      }
    } else if rank_sorted_by_count[0].1 == 3 {
      Hand::ThreeOfAKind {
        rank: rank_sorted_by_count[0].0,
        kickers: [rank_sorted_by_count[1].0, rank_sorted_by_count[2].0],
      }
    } else if rank_sorted_by_count[0].1 == 2 && rank_sorted_by_count[1].1 == 2 {
      Hand::TwoPair {
        high_pair: rank_sorted_by_count[0].0,
        low_pair: rank_sorted_by_count[1].0,
        kicker: rank_sorted_by_count[2].0,
      }
    } else if rank_sorted_by_count[0].1 == 2 {
      Hand::OnePair {
        rank: rank_sorted_by_count[0].0,
        kickers: [
          rank_sorted_by_count[1].0,
          rank_sorted_by_count[2].0,
          rank_sorted_by_count[3].0,
        ],
      }
    } else {
      // Further logic to determine other hand types (pairs, three of a kind, etc.) would go here.
      Hand::HighCard {
        cards: [
          all_cards[0].rank,
          all_cards[1].rank,
          all_cards[2].rank,
          all_cards[3].rank,
          all_cards[4].rank,
        ],
      }
    }
  }
}
