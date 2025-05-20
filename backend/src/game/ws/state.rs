use std::{collections::HashMap, sync::Arc};

use rocket::tokio::sync::{
  Mutex,
  mpsc::{Receiver, Sender, channel},
};
use uuid::Uuid;

use super::{
  commands::GameCommand,
  messages::{GameMessage, PlayerInfo, RoomInfo},
};

pub struct GameState {
  rooms: HashMap<Uuid, Arc<Mutex<Room>>>,
}

pub struct Room {
  id: Uuid,
  players: HashMap<Uuid, Player>,
}

struct Player {
  id: Uuid,
  name: String,
  sender: Sender<GameMessage>,
}

impl GameState {
  pub fn new() -> Arc<Mutex<Self>> {
    Arc::new(Mutex::new(Self {
      rooms: HashMap::new(),
    }))
  }

  pub async fn join_room(
    &mut self,
    id: Uuid,
    name: String,
    player: Uuid,
  ) -> (Receiver<GameMessage>, Arc<Mutex<Room>>) {
    let room = self.rooms.entry(id).or_insert_with(|| {
      Arc::new(Mutex::new(Room {
        id,
        players: HashMap::new(),
      }))
    });

    let (sender, receiver) = channel(100);
    let mut room_guard = room.lock().await;

    room_guard.players.insert(
      player,
      Player {
        id: player,
        name,
        sender: sender.clone(),
      },
    );
    let _ = sender.send(room_guard.room_info()).await;

    (receiver, room.clone())
  }

  pub async fn leave_room(&mut self, id: Uuid, player: Uuid) {
    if let Some(room) = self.rooms.get(&id) {
      let mut room = room.lock().await;
      room.players.remove(&player);
      if room.players.is_empty() {
        drop(room);
        self.rooms.remove(&id);
      }
    }
  }
}

impl Room {
  pub async fn handle_cmd(&self, player: Uuid, cmd_json: String) {
    let cmd: GameCommand = match serde_json::from_str(&cmd_json) {
      Ok(cmd) => cmd,
      Err(_) => {
        self
          .send_msg_to_player(player, GameMessage::InvalidCommand(cmd_json))
          .await;
        return;
      }
    };

    match cmd {
      GameCommand::RoomInfo => {
        self.send_msg_to_player(player, self.room_info()).await;
      }
    }
  }

  async fn send_msg_to_all(&self, msg: GameMessage) {
    for player in self.players.values() {
      let _ = player.sender.send(msg.clone()).await;
    }
  }

  async fn send_msg_to_player(&self, player: Uuid, msg: GameMessage) {
    if let Some(player) = self.players.get(&player) {
      let _ = player.sender.send(msg).await;
    }
  }

  fn room_info(&self) -> GameMessage {
    GameMessage::RoomInfo(RoomInfo {
      id: self.id,
      players: self
        .players
        .values()
        .map(|player| PlayerInfo {
          id: player.id,
          name: player.name.clone(),
        })
        .collect(),
    })
  }
}
