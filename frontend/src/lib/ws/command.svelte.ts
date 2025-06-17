type Command = RoomInfoCommand;

enum CommandType {
  RoomInfo = 'RoomInfo'
}

type RoomInfoCommand = {
  type: CommandType.RoomInfo;
};
