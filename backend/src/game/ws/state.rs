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
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  db::tables::Tables,
  error::Result,
  game::ws::{
    game_state::GameState,
    messages::{InvalidCommand, PlayerTurn, YourCards},
  },
};

use super::{
  commands::GameCommand,
  messages::{GameMessage, PlayerInfo, RoomInfo},
};

type Rooms = Arc<Mutex<HashMap<Uuid, Arc<Mutex<Room>>>>>;
type Players = Arc<Mutex<HashMap<Uuid, Player>>>;

pub struct GlobalGameState {
  nats_url: String,
  nats_update_subject: String,
  rooms: Rooms,
}

pub struct Room {
  id: Uuid,
  players: Players,
  game_state: Option<GameState>,
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

#[derive(Serialize, Deserialize, Clone)]
struct NatsMessage {
  receiver: Option<Uuid>,
  msg: GameMessage,
}

impl GlobalGameState {
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
  ) -> Result<(Receiver<GameMessage>, Arc<Mutex<Room>>)> {
    let nats_url = self.nats_url.clone();
    let nats_update_subject = Subject::from(format!("{}.{}", self.nats_update_subject, id));
    let nats_update_subject_ = nats_update_subject.clone();

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
            let Ok(msg) = serde_json::from_slice::<NatsMessage>(&msg.payload) else {
              log::warn!("Failed to parse message: {:?}", msg);
              return;
            };
            let players = players_.lock().await;
            if let Some(receiver) = msg.receiver {
              if let Some(player) = players.get(&receiver) {
                let _ = player.sender.send(msg.msg.clone()).await;
              }
            } else {
              for player in players.values() {
                let _ = player.sender.send(msg.msg.clone()).await;
              }
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
    let room_arc = rooms.entry(id).or_insert_with(|| {
      Arc::new(Mutex::new(Room {
        id,
        players,
        game_state: None,
        nats_task,
        nats_notify,
        nats_sender,
        nats_update_subject,
      }))
    });
    let room = room_arc.lock().await;

    room
      .send_msg_to_all(GameMessage::PlayerJoined(PlayerInfo {
        id: player,
        name: name.clone(),
      }))
      .await;

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
    drop(players);

    let _ = sender.send(room.room_info(db).await?).await;
    if let Some(game_state) = &room.game_state {
      let _ = sender.send(game_state.game_info()).await;
    }

    Ok((receiver, room_arc.clone()))
  }

  pub async fn leave_room(&self, id: Uuid, player: Uuid) {
    let mut rooms = self.rooms.lock().await;
    if let Some(room) = rooms.get(&id) {
      let mut room = room.lock().await;
      if let Some(game_state) = &mut room.game_state {
        for msg in game_state.player_leaves(player) {
          room.send_msg_to_all(msg).await;
        }
      }
      let mut players = room.players.lock().await;
      players.remove(&player);
      if players.is_empty() {
        drop(players);
        drop(room);
        let Some(room) = rooms.remove(&id) else {
          log::warn!("Failed to remove room");
          return;
        };
        let room = room.lock().await;
        room.nats_notify.notify_one();
        room.nats_task.abort();
      } else {
        room
          .send_msg_to_all(GameMessage::PlayerLeft(PlayerInfo {
            id: player,
            name: "".to_string(), // Name is not needed here
          }))
          .await;
      }
    }
  }
}

impl Room {
  pub async fn handle_cmd(
    &mut self,
    player: Uuid,
    cmd_json: String,
    db: &Tables<'_>,
  ) -> Result<()> {
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

    match cmd {
      GameCommand::RoomInfo => {
        self
          .send_msg_to_current_player(player, self.room_info(db).await?)
          .await;
      }
      GameCommand::GameInfo => {
        if let Some(game_state) = &self.game_state {
          self
            .send_msg_to_current_player(player, game_state.game_info())
            .await;
        } else {
          self
            .send_msg_to_current_player(
              player,
              GameMessage::InvalidCommand(InvalidCommand {
                message: "Game not started".to_string(),
              }),
            )
            .await;
        }
      }
      GameCommand::BackToLobby => {
        if let Some(game_state) = &self.game_state {
          let room = db.room().get(self.id).await?;
          if room.owner != player || !game_state.game_ended {
            self
              .send_msg_to_current_player(
                player,
                GameMessage::InvalidCommand(InvalidCommand {
                  message: "Only the room owner can end the game".to_string(),
                }),
              )
              .await;
            return Ok(());
          }
          // Reset game state
          self.game_state = None;

          // Notify players
          self.send_msg_to_all(GameMessage::BackToLobby).await;
        } else {
          self
            .send_msg_to_current_player(
              player,
              GameMessage::InvalidCommand(InvalidCommand {
                message: "Game not started".to_string(),
              }),
            )
            .await;
        }
      }
      GameCommand::StartGame => {
        if self.game_state.is_some() {
          self
            .send_msg_to_current_player(
              player,
              GameMessage::InvalidCommand(InvalidCommand {
                message: "Game already started".to_string(),
              }),
            )
            .await;
          return Ok(());
        }

        let players = self.players.lock().await;
        if players.len() < 2 {
          drop(players);
          self
            .send_msg_to_current_player(
              player,
              GameMessage::InvalidCommand(InvalidCommand {
                message: "Not enough players to start the game".to_string(),
              }),
            )
            .await;
          return Ok(());
        }

        // Check if the player is the owner
        let room = db.room().get(self.id).await?;
        if room.owner != player {
          drop(players);
          self
            .send_msg_to_current_player(
              player,
              GameMessage::InvalidCommand(InvalidCommand {
                message: "Only the room owner can start the game".to_string(),
              }),
            )
            .await;
          return Ok(());
        }

        // Initialize game state here
        let state = GameState::init(
          players.keys().copied().collect::<Vec<_>>(),
          room.small_blind,
          room.starting_cash,
        );

        drop(players);
        self.send_msg_to_all(state.game_info()).await;

        for player in state.players.values() {
          self
            .send_msg_to_player(
              player.id,
              GameMessage::YourCards(YourCards {
                cards: player.cards,
              }),
            )
            .await;
        }

        self
          .send_msg_to_all(GameMessage::PlayerTurn(PlayerTurn {
            id: state.current_player,
          }))
          .await;

        self.game_state = Some(state);
      }
      GameCommand::NextRound => {
        if let Some(game_state) = &mut self.game_state {
          for msg in game_state.next_round() {
            self.send_msg_to_all(msg).await;
          }
        } else {
          self
            .send_msg_to_current_player(
              player,
              GameMessage::InvalidCommand(InvalidCommand {
                message: "Game not started".to_string(),
              }),
            )
            .await;
          return Ok(());
        }
      }
      cmd => {
        if let Some(game_state) = &mut self.game_state {
          for msg in game_state.handle_cmd(cmd, player) {
            self.send_msg_to_all(msg).await;
          }
        } else {
          self
            .send_msg_to_current_player(
              player,
              GameMessage::InvalidCommand(InvalidCommand {
                message: "Game not started".to_string(),
              }),
            )
            .await;
          return Ok(());
        }
      }
    }

    Ok(())
  }

  async fn send_msg_to_player(&self, player: Uuid, msg: GameMessage) {
    self.send_msg(msg, Some(player)).await;
  }

  async fn send_msg_to_all(&self, msg: GameMessage) {
    self.send_msg(msg, None).await;
  }

  async fn send_msg(&self, msg: GameMessage, receiver: Option<Uuid>) {
    let Ok(msg) = serde_json::to_vec(&NatsMessage { msg, receiver }) else {
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
