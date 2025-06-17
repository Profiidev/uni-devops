type Message = RoomInfo | InvalidCommand;

enum MessageType {
  InvalidCommand = 'InvalidCommand',
  RoomInfo = 'RoomInfo'
}

type RoomInfo = {
  type: MessageType.RoomInfo;
  id: string; // uuid
  players: PlayerInfo[];
};

type PlayerInfo = {
  id: string; // uuid
  name: string;
};

type InvalidCommand = {
  type: MessageType.InvalidCommand;
  message: string;
};
