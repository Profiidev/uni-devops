<script lang="ts">
  import { onMount } from 'svelte';
  import {
    connectWebSocket,
    closeWebSocket,
    sendWebSocketMessage
  } from '$lib/ws/websocket.svelte';
  import {
    MessageType,
    type RoomInfo,
    type PlayerInfo,
    type Card,
    type RoundEnded,
    type PlayerGameInfo,
    type Message,
    type GameState,
    PlayerActionType,
    type GameEnded,
    type PlayerAction
  } from '$lib/ws/message.svelte';
  import { userInfo } from '$lib/user.svelte.js';
  import { CommandType } from '$lib/ws/command.svelte.js';
  import PlayerComponent from './playerComponent.svelte';
  import CardComponent from './cardComponent.svelte';

  const { data } = $props();
  const { gameId } = data;

  let roomInfo: RoomInfo | null = $state(null);
  let ownPlayerId: string | null = $state(null);
  let gameState: GameState | null = $state(null);
  let ownPlayer: () => PlayerGameInfo | null = $derived(() => {
    if (!gameState || !ownPlayerId) return null;
    const ownId = ownPlayerId;
    const ownPlayer = gameState.players.find((p) => p.id === ownId);
    if (ownPlayer) return ownPlayer;
    return gameState.players[0];
  });
  let enemyPlayer: () => PlayerGameInfo[] = $derived(() => {
    const playerBottom = ownPlayer();
    if (!gameState || !playerBottom) return [];
    const ps = gameState.players;
    const ownId = playerBottom.id;
    const index = ps.findIndex((p) => p.id === ownId);
    if (index === -1) return [];
    return [...ps.slice(index + 1), ...ps.slice(0, index)];
  });

  let ownCards: Card[] = $state([]);
  let paddedCards: () => Card[] | undefined[] = $derived(() => {
    if (!gameState) return Array(5).fill(undefined);
    return Array.from(
      { length: 5 },
      (_, i) => gameState?.revealed_cards[i] || undefined
    );
  });
  let raiseAmount: number = $state(1);
  let pot: number = $state(0);
  let callAmount: number = $state(0);
  let maxRaiseAmount: () => number = $derived(() => {
    if (!ownPlayer()) return 0;
    return Math.max(
      (ownPlayer()?.cash || 0) + (ownPlayer()?.bet || 0) - callAmount,
      0
    );
  });
  let lastPlayerAction: { action: PlayerAction; id: string }[] = $state([]);
  let roundEnded: RoundEnded | null = $state(null);
  let gameEnded: GameEnded | null = $state(null);

  function handleIncoming(data: Message) {
    console.log('Received data:', data);

    switch (data.type) {
      case MessageType.RoomInfo:
        roomInfo = data;
        for (const player of roomInfo.players) player.online = true;
        break;
      case MessageType.InvalidCommand:
        console.error('Invalid command received:', data);
        break;
      case MessageType.GameInfo:
        gameState = data;
        break;
      case MessageType.GameStarted:
        roundEnded = null;
        gameEnded = null;
        gameState = data;
        if (roomInfo) {
          callAmount = roomInfo.small_blind * 2;
          roomInfo.game_running = true;
          pot = roomInfo.small_blind * 3;
        }
        break;
      case MessageType.PlayerJoined:
        if (!roomInfo) break;
        const playerJoined = roomInfo.players.find((p) => p.id === data.id);
        if (playerJoined) {
          playerJoined.name = data.name;
          playerJoined.online = true;
          break;
        }
        roomInfo.players = [
          ...(roomInfo.players || []),
          { id: data.id, name: data.name, online: true }
        ];
        break;
      case MessageType.PlayerLeft:
        if (!roomInfo) break;
        const playerLeft = roomInfo.players.find((p) => p.id === data.id);
        if (playerLeft) playerLeft.online = false;
        break;
      case MessageType.PlayerTurn:
        if (!gameState) break;
        gameState.current_player = data.id;
        lastPlayerAction = lastPlayerAction.filter(
          (action) => action.id !== data.id
        );
        break;
      case MessageType.YourCards:
        ownCards = data.cards;
        break;
      case MessageType.CardRevealed:
        if (!gameState) break;
        gameState.revealed_cards = [
          ...gameState.revealed_cards,
          { rank: data.rank, suit: data.suit }
        ];
        break;
      case MessageType.PlayerActed:
        if (!gameState) break;
        const player = gameState.players.find((p) => p.id === data.id);
        if (!player) break;
        lastPlayerAction = lastPlayerAction.filter(
          (action) => action.id !== data.id
        );
        lastPlayerAction.push({ action: data.action, id: data.id });
        switch (data.action.type) {
          case PlayerActionType.Fold:
            player.folded = true;
            break;
          case PlayerActionType.Check:
            break;
          case PlayerActionType.Raise:
            callAmount += data.action.amount;
          // fallthrough intentional
          case PlayerActionType.Call:
            const difference = Math.min(callAmount - player.bet, player.cash);
            player.bet += difference;
            player.cash -= difference;
            pot += difference;
            raiseAmount = Math.min(raiseAmount, maxRaiseAmount());
            break;
        }
        break;
      case MessageType.RoundEnded:
        if (!roomInfo || !gameState) break;
        gameState.round_running = false;
        roundEnded = data;
        pot = 0;
        gameState.current_player = 'null';
        lastPlayerAction = [];
        for (const player of roundEnded.players) {
          const playerInGameState = gameState.players.find(
            (p) => p.id === player.id
          );
          if (playerInGameState) {
            playerInGameState.cash += player.winnings;
            playerInGameState.bet = 0;
          }
        }
        break;
      case MessageType.GameEnded:
        gameEnded = data;
        if (gameState) gameState.game_ended = true;
        break;
      case MessageType.BackToLobby:
        if (roomInfo) roomInfo.game_running = false;
        break;
    }
  }

  onMount(() => {
    userInfo().then((playerInfo) => {
      if (playerInfo !== null) {
        ownPlayerId = (playerInfo as PlayerInfo).id;
        console.log('Own ID:', ownPlayerId);
        connectWebSocket(gameId, handleIncoming);
      }
    });

    return () => {
      closeWebSocket();
    };
  });
</script>

{#if !roomInfo}
  <p>Loading room information...</p>
{:else if !roomInfo.game_running}
  <main>
    <h1>Dealing With DevOps</h1>

    <div class="middle">
      <h2>Invite:</h2>
      <div class="mid">
        <div class="copyLink">
          <button
            onclick={() => navigator.clipboard.writeText(window.location.href)}
            >copy Link</button
          >
          <p>{window.location.href}</p>
        </div>
      </div>

      <h2>Player:</h2>
      <div class="mid">
        {#each roomInfo.players as player}
          {#if player.online}
            <p>{player.name}</p>
          {/if}
        {/each}

        {#if roomInfo.owner === ownPlayerId}
          <button
            class="primaryButton"
            onclick={() =>
              sendWebSocketMessage({ type: CommandType.StartGame })}
          >
            Start Game
          </button>
        {/if}
      </div>
    </div>
  </main>
{:else if gameState}
  <div class="enemyPlayerContainer">
    {#if enemyPlayer().length > 0}
      {#each enemyPlayer() as player}
        <PlayerComponent
          name={roomInfo.players.find((p) => p.id === player.id)?.name ||
            'Unknown Player'}
          playerGameInfo={player}
          cards={roundEnded?.players.find((p) => p.id === player.id)?.cards || [
            undefined,
            undefined
          ]}
          turn={gameState.current_player === player.id}
          hand={roundEnded?.players.find((p) => p.id === player.id)
            ?.best_hand || null}
          winnings={roundEnded?.players.find((p) => p.id === player.id)
            ?.winnings || null}
          winner={gameEnded?.winner === player.id}
          lastAction={lastPlayerAction.find((action) => action.id === player.id)
            ?.action || null}
          roundRunning={gameState.round_running}
        />
      {/each}
    {/if}
  </div>

  <div class="center">
    <p class="pot">
      {#if !roundEnded}
        Pot: {pot}
      {/if}
    </p>
    <div class="centerCards">
      {#each paddedCards() as card}
        <CardComponent {card} />
      {/each}
    </div>
  </div>

  <div class="ownPlayerContainer">
    {#if ownPlayerId}
      {@const player = ownPlayer()}
      {#if player}
        <PlayerComponent
          name={roomInfo.players.find((p) => p.id === player.id)?.name ||
            'Unknown Player'}
          playerGameInfo={player}
          cards={ownCards || [undefined, undefined]}
          turn={gameState?.current_player === player.id}
          hand={roundEnded?.players.find((p) => p.id === player.id)
            ?.best_hand || null}
          winnings={roundEnded?.players.find((p) => p.id === player.id)
            ?.winnings || null}
          winner={gameEnded?.winner === player.id}
          lastAction={lastPlayerAction.find((action) => action.id === player.id)
            ?.action || null}
          roundRunning={gameState.round_running}
        />
      {/if}
    {/if}
  </div>

  <div class="actions">
    {#if gameState.round_running}
      {#if !ownPlayer()?.folded && ownPlayer()?.id === ownPlayerId}
        <div
          class="gameActions {gameState?.current_player === ownPlayerId
            ? 'yourTurn'
            : ''}"
        >
          <button
            onclick={() => sendWebSocketMessage({ type: CommandType.Fold })}
          >
            Fold
          </button>
          {#if callAmount > (ownPlayer()?.bet || 0) && (ownPlayer()?.cash || 0) > 0}
            <button
              onclick={() => sendWebSocketMessage({ type: CommandType.Call })}
            >
              Call
            </button>
          {:else}
            <button
              onclick={() => sendWebSocketMessage({ type: CommandType.Check })}
            >
              Check
            </button>
          {/if}

          <div
            class="raiseContainer {maxRaiseAmount() > 0
              ? 'possibleToRaise'
              : ''}"
          >
            <input
              type="range"
              class="raiseSlider"
              bind:value={raiseAmount}
              min="1"
              max={maxRaiseAmount()}
              step="1"
            />
            <input
              type="number"
              class="raiseInput"
              bind:value={raiseAmount}
              min="1"
              max={maxRaiseAmount()}
            />
            <button
              onclick={() => {
                sendWebSocketMessage({
                  type: CommandType.Raise,
                  amount: raiseAmount
                });
              }}
            >
              Raise
            </button>
          </div>
        </div>
      {/if}
    {:else if roomInfo?.owner === ownPlayerId}
      {#if gameState.game_ended}
        <button
          onclick={() =>
            sendWebSocketMessage({ type: CommandType.BackToLobby })}
        >
          Back to Lobby
        </button>
      {:else}
        <button
          onclick={() => sendWebSocketMessage({ type: CommandType.NextRound })}
        >
          Next Round
        </button>
      {/if}
    {/if}
  </div>
{:else}
  <p>Loading game information...</p>
{/if}
<br />
<br />
<br />
<br />
<br />
<p>only there for testing:</p>
<div
  class="tmpOptions"
  style="display: flex; gap: 20px; flex-wrap: wrap; margin: 20px;"
>
  <button onclick={() => sendWebSocketMessage({ type: CommandType.StartGame })}
    >Start Game</button
  >
  <button onclick={() => sendWebSocketMessage({ type: CommandType.NextRound })}>
    Next Round
  </button>
  <button
    onclick={() => sendWebSocketMessage({ type: CommandType.BackToLobby })}
  >
    Back to Lobby
  </button>
  <button onclick={() => sendWebSocketMessage({ type: CommandType.GameInfo })}>
    Refresh Game Info
  </button>
  <button onclick={() => sendWebSocketMessage({ type: CommandType.RoomInfo })}>
    Refresh Room Info
  </button>
</div>

<style>
  .enemyPlayerContainer {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    gap: 10px;
  }

  .ownPlayerContainer {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .center {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    gap: 10px;
    margin: 20px;
  }

  .centerCards {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }

  .actions {
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 20px;

    button {
      padding: 5px 10px;
      background-color: var(--primary-color);
      border: 3px solid var(--surface-background);
      border-radius: 10px;
      min-width: 80px;
      font-size: large;
    }

    .gameActions {
      display: flex;
      flex-direction: row;
      flex-wrap: wrap;
      gap: 30px;
      opacity: 0.6;

      .raiseContainer {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 10px;
        opacity: 0.6;

        button {
          cursor: default;
        }
      }

      .possibleToRaise {
        opacity: 1;

        button {
          cursor: pointer;
        }
      }
    }

    .yourTurn {
      opacity: 1;

      button {
        cursor: pointer;
      }
    }
  }

  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 2rem;
    gap: 2rem;
  }

  .middle {
    border-radius: 15px;
    padding: 3.5rem;
    gap: 1rem;
    background-color: var(--surface-background);
    width: min(90vw, 700px);
  }

  .mid {
    display: flex;
    justify-content: center;
    flex-direction: column;
    align-items: center;
    margin: 10px;
    gap: 10px;
  }

  h1 {
    font-size: 2rem;
  }

  h2 {
    font-size: 1.5rem;
  }

  p {
    font-size: 1rem;
  }

  .primaryButton {
    padding: 0.5rem 1rem;
    width: fit-content;
    background-color: var(--primary-color);
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .copyLink {
    display: flex;
    flex-direction: row;
    gap: 10px;
    border: 2px solid var(--text-color);
    border-radius: 10px;
    align-items: center;
    width: fit-content;

    button {
      cursor: pointer;
      background-color: var(--primary-color);
      padding: 10px;
      border-radius: 10px;
    }

    p {
      max-width: 220px;
      overflow-y: scroll;
      white-space: nowrap;
    }
  }
</style>
