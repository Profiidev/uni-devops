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
  | GameEnded;

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
  GameEnded = 'GameEnded'
}

export type RoomInfo = {
  type: MessageType.RoomInfo;
  id: string; // uuid
  players: PlayerInfo[];
};

export type PlayerInfo = {
  id: string; // uuid
  name: string;
};

export type InvalidCommand = {
  type: MessageType.InvalidCommand;
  message: string;
};

export type GameInfo = {
  type: MessageType.GameInfo;
  pot: number;
  players: PlayerGameInfo[];
  currentPlayer: string; // uuid
  smallBlind: string; // uuid
};

export type PlayerGameInfo = {
  id: string; // uuid
  cash: number;
  folded: boolean;
};

export type GameStarted = {
  type: MessageType.GameStarted;
  smallBlind: string; // uuid
};

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

export type GameEnded = {
  type: MessageType.GameEnded;
  winnings: [string, number][]; // uuid and amount won
  revealedCards: [string, [Card, Card]][]; // uuid and cards
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
