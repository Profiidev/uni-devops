export type Message =
  | RoomInfo
  | InvalidCommand
  | GameInfo
  | GameStarted
  | PlayerJoined
  | PlayerLeft
  | PlayerTurn
  | YourCards
  | CardRevealed
  | PlayerActed
  | RoundEnded
  | GameEnded
  | BackToLobby
  | Heartbeat;

export enum MessageType {
  InvalidCommand = 'InvalidCommand',
  RoomInfo = 'RoomInfo',
  GameInfo = 'GameInfo',
  GameStarted = 'GameStarted',
  PlayerJoined = 'PlayerJoined',
  PlayerLeft = 'PlayerLeft',
  PlayerTurn = 'PlayerTurn',
  YourCards = 'YourCards',
  CardRevealed = 'CardRevealed',
  PlayerActed = 'PlayerActed',
  RoundEnded = 'RoundEnded',
  GameEnded = 'GameEnded',
  BackToLobby = 'BackToLobby',
  Heartbeat = 'Heartbeat'
}

export type RoomInfo = {
  type: MessageType.RoomInfo;
  id: string; // uuid
  players: PlayerInfo[];
  owner: string; // uuid
  starting_cash: number;
  small_blind: number;
  game_running: boolean;
};

export type PlayerInfo = {
  id: string; // uuid
  name: string;
};

export type InvalidCommand = {
  type: MessageType.InvalidCommand;
  message: string;
};

export type GameState = {
  pot: number;
  players: PlayerGameInfo[];
  current_player: string; // uuid
  small_blind: string; // uuid
  revealed_cards: Card[];
};

export type GameInfo = {
  type: MessageType.GameInfo;
} & GameState;

export type PlayerGameInfo = {
  id: string; // uuid
  cash: number;
  bet: number;
  folded: boolean;
};

export type GameStarted = {
  type: MessageType.GameStarted;
} & GameState;

export type PlayerJoined = {
  type: MessageType.PlayerJoined;
} & PlayerInfo;

export type PlayerLeft = {
  type: MessageType.PlayerLeft;
} & PlayerInfo;

export type PlayerTurn = {
  type: MessageType.PlayerTurn;
  id: string; // uuid
};

export type YourCards = {
  type: MessageType.YourCards;
  cards: [Card, Card];
};

export type CardRevealed = {
  type: MessageType.CardRevealed;
} & Card;

export type PlayerActed = {
  type: MessageType.PlayerActed;
  id: string; // uuid
  action: PlayerAction;
};

export enum PlayerActionType {
  Fold = 'Fold',
  Call = 'Call',
  Raise = 'Raise',
  Check = 'Check'
}

export type PlayerAction =
  | {
      type: Exclude<PlayerActionType, PlayerActionType.Raise>; // Fold, Call, Check
    }
  | {
      type: PlayerActionType.Raise;
      amount: number;
    };

export type RoundEnded = {
  type: MessageType.RoundEnded;
  players: PlayerEndedInfo[];
};

export type PlayerEndedInfo = {
  id: string; // uuid
  cards?: [Card, Card]; // Only present if the player did not fold and more than 2 players are left because only then the cards are revealed
  best_hand?: Hand; // Same as cards
  winnings: number;
};

export type GameEnded = {
  type: MessageType.GameEnded;
  winner: string; // uuid
};

export type BackToLobby = {
  type: MessageType.BackToLobby;
};

export type Heartbeat = {
  type: MessageType.Heartbeat;
};

export type Card = {
  rank: Rank;
  suit: Suit;
};

export enum Rank {
  Two = 'Two',
  Three = 'Three',
  Four = 'Four',
  Five = 'Five',
  Six = 'Six',
  Seven = 'Seven',
  Eight = 'Eight',
  Nine = 'Nine',
  Ten = 'Ten',
  Jack = 'Jack',
  Queen = 'Queen',
  King = 'King',
  Ace = 'Ace'
}

export enum Suit {
  Hearts = 'Hearts',
  Diamonds = 'Diamonds',
  Clubs = 'Clubs',
  Spades = 'Spades'
}

export type Hand =
  | HighCard
  | OnePair
  | TwoPair
  | ThreeOfAKind
  | Straight
  | Flush
  | FullHouse
  | FourOfAKind
  | StraightFlush
  | RoyalFlush;

export enum HandType {
  HighCard = 'HighCard',
  OnePair = 'OnePair',
  TwoPair = 'TwoPair',
  ThreeOfAKind = 'ThreeOfAKind',
  Straight = 'Straight',
  Flush = 'Flush',
  FullHouse = 'FullHouse',
  FourOfAKind = 'FourOfAKind',
  StraightFlush = 'StraightFlush',
  RoyalFlush = 'RoyalFlush'
}

export type HighCard = {
  type: HandType.HighCard;
  cards: [Rank, Rank, Rank, Rank, Rank]; // 5 ranks in descending order
};

export type OnePair = {
  type: HandType.OnePair;
  rank: Rank;
  kickers: [Rank, Rank, Rank]; // 3 kickers in descending order
};

export type TwoPair = {
  type: HandType.TwoPair;
  highPair: Rank; // Higher pair
  lowPair: Rank; // Lower pair
  kicker: Rank; // kicker
};

export type ThreeOfAKind = {
  type: HandType.ThreeOfAKind;
  rank: Rank;
  kickers: [Rank, Rank]; // 2 kickers in descending order
};

export type Straight = {
  type: HandType.Straight;
  rank: Rank; // Highest rank in the straight
};

export type Flush = {
  type: HandType.Flush;
  suit: Suit; // Suit of the flush
};

export type FullHouse = {
  type: HandType.FullHouse;
  triplet: Rank; // Rank of the triplet
  pair: Rank; // Rank of the pair
};

export type FourOfAKind = {
  type: HandType.FourOfAKind;
  rank: Rank; // Rank of the four of a kind
  kicker: Rank; // Kicker
};

export type StraightFlush = {
  type: HandType.StraightFlush;
  rank: Rank; // Highest rank in the straight flush
  suit: Suit; // Suit of the straight flush
};

export type RoyalFlush = {
  type: HandType.RoyalFlush;
  suit: Suit; // Suit of the royal flush
};
