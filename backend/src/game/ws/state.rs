use std::{collections::HashMap, sync::Arc};

use async_nats::{Client, Subject, connect};
use rocket::{
  futures::StreamExt,
  tokio::{
    select, spawn,
    sync::{
      Mutex, Notify,
      mpsc::{Receiver, Sender, channel},
    },
    task::JoinHandle,
  },
};
use uuid::Uuid;

use crate::{db::tables::Tables, error::Result, game::ws::messages::InvalidCommand};

use super::{
  commands::GameCommand,
  messages::{GameMessage, PlayerInfo, RoomInfo},
};

type Rooms = Arc<Mutex<HashMap<Uuid, Arc<Room>>>>;
type Players = Arc<Mutex<HashMap<Uuid, Player>>>;

pub struct GameState {
  nats_url: String,
  nats_update_subject: String,
  rooms: Rooms,
}

pub struct Room {
  id: Uuid,
  players: Players,
  nats_task: JoinHandle<()>,
  nats_notify: Arc<Notify>,
  nats_sender: Client,
  nats_update_subject: Subject,
}

struct Player {
  id: Uuid,
  name: String,
  sender: Sender<GameMessage>,
}

impl GameState {
  pub async fn new() -> Self {
    let nats_url = std::env::var("NATS_URL").expect("Failed to load NATS_URL");
    let nats_update_subject =
      std::env::var("NATS_UPDATE_SUBJECT").expect("Failed to load NATS_UPDATE_SUBJECT");

    let rooms: Rooms = Default::default();

    Self {
      rooms,
      nats_url,
      nats_update_subject,
    }
  }

  pub async fn join_room(
    &self,
    id: Uuid,
    name: String,
    player: Uuid,
    db: &Tables<'_>,
  ) -> Result<(Receiver<GameMessage>, Arc<Room>)> {
    let nats_url = self.nats_url.clone();
    let nats_update_subject_ = format!("{}.{}", self.nats_update_subject, id);
    let nats_update_subject = Subject::from(nats_update_subject_.as_str());

    let players: Players = Default::default();
    let players_ = players.clone();

    let nats_sender = connect(&nats_url).await.expect("Failed to connect to nats");
    let nats_notify = Arc::new(Notify::new());
    let nats_notify_ = nats_notify.clone();

    let nats_task = spawn(async move {
      let client = connect(&nats_url).await.expect("Failed to connect to nats");
      let mut sub = client
        .subscribe(nats_update_subject_)
        .await
        .expect("Failed to subscribe to subject");

      loop {
        select! {
          _ = nats_notify_.notified() => {
            log::info!("NATS notify received, exiting task");
            return;
          }
          Some(msg) = sub.next() => {
            let Ok(msg) = serde_json::from_slice::<GameMessage>(&msg.payload) else {
              log::warn!("Failed to parse message: {:?}", msg);
              return;
            };
            let players = players_.lock().await;
            for player in players.values() {
              let _ = player.sender.send(msg.clone()).await;
            }
          }
          else => {
            log::info!("NATS subscription closed, exiting task");
            return;
          }
        }
      }
    });

    let mut rooms = self.rooms.lock().await;
    let room = rooms.entry(id).or_insert_with(|| {
      Arc::new(Room {
        id,
        players,
        nats_task,
        nats_notify,
        nats_sender,
        nats_update_subject,
      })
    });

    let (sender, receiver) = channel(100);
    let mut players = room.players.lock().await;

    players.insert(
      player,
      Player {
        id: player,
        name,
        sender: sender.clone(),
      },
    );
    let _ = sender.send(room.room_info(db).await?).await;

    Ok((receiver, room.clone()))
  }

  pub async fn leave_room(&self, id: Uuid, player: Uuid) {
    let mut rooms = self.rooms.lock().await;
    if let Some(room) = rooms.get(&id) {
      let mut players = room.players.lock().await;
      players.remove(&player);
      if players.is_empty() {
        drop(players);
        let Some(room) = rooms.remove(&id) else {
          log::warn!("Failed to remove room");
          return;
        };
        room.nats_notify.notify_one();
        room.nats_task.abort();
      }
    }
  }
}

impl Room {
  pub async fn handle_cmd(&self, player: Uuid, cmd_json: String, db: &Tables<'_>) -> Result<()> {
    let cmd: GameCommand = match serde_json::from_str(&cmd_json) {
      Ok(cmd) => cmd,
      Err(_) => {
        self
          .send_msg_to_current_player(
            player,
            GameMessage::InvalidCommand(InvalidCommand { message: cmd_json }),
          )
          .await;
        return Ok(());
      }
    };

    if let GameCommand::RoomInfo = cmd {
      self
        .send_msg_to_current_player(player, self.room_info(db).await?)
        .await;
    }

    Ok(())
  }

  #[allow(unused)]
  async fn send_msg_to_all(&self, msg: GameMessage) {
    let Ok(msg) = serde_json::to_vec(&msg) else {
      log::warn!("Failed to serialize message");
      return;
    };

    self
      .nats_sender
      .publish(self.nats_update_subject.clone(), msg.into())
      .await
      .expect("Failed to publish message");
  }

  async fn send_msg_to_current_player(&self, player: Uuid, msg: GameMessage) {
    let players = self.players.lock().await;
    if let Some(player) = players.get(&player) {
      let _ = player.sender.send(msg).await;
    }
  }

  async fn room_info(&self, db: &Tables<'_>) -> Result<GameMessage> {
    let players = self.players.lock().await;
    let room = db.room().get(self.id).await?;

    Ok(GameMessage::RoomInfo(RoomInfo {
      id: self.id,
      players: players
        .values()
        .map(|player| PlayerInfo {
          id: player.id,
          name: player.name.clone(),
        })
        .collect(),
      game_running: room.game_running,
      starting_cash: room.starting_cash,
      small_blind: room.small_blind,
      owner: room.owner,
    }))
  }
}
