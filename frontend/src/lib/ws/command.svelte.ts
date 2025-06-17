export type Command =
  | RoomInfoCommand
  | GameInfoCommand
  | StartGameCommand
  | FoldCommand
  | CallCommand
  | RaiseCommand
  | CheckCommand
  | BackToLobbyCommand;

export enum CommandType {
  RoomInfo = 'RoomInfo',
  GameInfo = 'GameInfo',
  StartGame = 'StartGame',
  Fold = 'Fold',
  Call = 'Call',
  Raise = 'Raise',
  Check = 'Check',
  BackToLobby = 'BackToLobby'
}

export type RoomInfoCommand = {
  type: CommandType.RoomInfo;
};

export type GameInfoCommand = {
  type: CommandType.GameInfo;
};

export type StartGameCommand = {
  type: CommandType.StartGame;
};

export type FoldCommand = {
  type: CommandType.Fold;
};

export type CallCommand = {
  type: CommandType.Call;
};

export type RaiseCommand = {
  type: CommandType.Raise;
  amount: number; // The amount to raise
};

export type CheckCommand = {
  type: CommandType.Check;
};

export type BackToLobbyCommand = {
  type: CommandType.BackToLobby;
};
