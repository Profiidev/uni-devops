<script lang="ts">
  import { onMount } from 'svelte';
  import {
    connectWebSocket,
    closeWebSocket,
    sendWebSocketMessage
  } from '$lib/websocket.svelte';
  import {
    MessageType,
    type RoomInfo,
    type PlayerInfo,
    type Card,
    type RoundEnded,
    type PlayerGameInfo,
    type Message,
    type GameState,
    PlayerActionType
  } from '$lib/ws/message.svelte';
  import { userInfo } from '$lib/user.svelte.js';
  import { CommandType } from '$lib/ws/command.svelte.js';
  import Player from './player.svelte';

  const { data } = $props();
  const { gameId } = data;

  let roomInfo: RoomInfo | null = $state(null);
  let ownPlayerInfo: PlayerInfo | null = $state(null);
  let gameState: GameState | null = $state(null);
  let ownPlayer: () => PlayerGameInfo | null = $derived(() => {
    if (!gameState || !ownPlayerInfo) return null;
    const ownId = ownPlayerInfo.id;
    return gameState.players.find((p) => p.id === ownId) || null;
  });
  let enemyPlayer: () => PlayerGameInfo[] = $derived(() => {
    if (!gameState || !ownPlayerInfo) return [];
    const ps = gameState.players;
    const ownId = ownPlayerInfo.id;
    const index = ps.findIndex((p) => p.id === ownId);
    if (index === -1) return [];
    return [...ps.slice(index + 1), ...ps.slice(0, index)];
  });

  let ownCards: Card[] = $state([]);
  let raiseAmount: number = $state(0);
  let revealedCards: Card[] = $state([]);
  let paddedCards = $derived(() =>
    Array.from(
      { length: 5 },
      (_, i) => revealedCards[i] || { rank: '?', suit: '?' }
    )
  );
  let callAmount: number = 0;
  let roundEnded: RoundEnded | null = $state(null);

  function handleIncoming(data: Message) {
    console.log('Received data:', data);

    switch (data.type) {
      case MessageType.RoomInfo:
        roomInfo = data;
        callAmount = data.small_blind * 2;
        break;
      case MessageType.InvalidCommand:
        console.error('Invalid command received:', data);
        break;
      case MessageType.GameInfo:
        gameState = data;
        if (roomInfo) roomInfo.game_running = true;
        break;
      case MessageType.GameStarted:
        gameState = data;
        if (roomInfo) roomInfo.game_running = true;
        break;
      case MessageType.PlayerJoined:
        if (!roomInfo) break;
        if (roomInfo.players.some((p) => p.id === data.id)) break;
        roomInfo.players = [
          ...(roomInfo.players || []),
          { id: data.id, name: data.name }
        ];
        break;
      case MessageType.PlayerLeft:
        if (!roomInfo) break;
        roomInfo.players = roomInfo.players.filter(
          (p: PlayerInfo) => p.id !== data.id
        );
        break;
      case MessageType.PlayerTurn:
        if (gameState) gameState.current_player = data.id;
        else sendWebSocketMessage({ type: CommandType.GameInfo });
        break;
      case MessageType.YourCards:
        ownCards = data.cards;
        break;
      case MessageType.CardRevealed:
        revealedCards = [
          ...revealedCards,
          { rank: data.rank, suit: data.suit }
        ];
        break;
      case MessageType.PlayerActed:
        if (!gameState) break;
        const player = gameState.players.find((p) => p.id === data.id);
        if (!player) break;
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
            break;
        }
        break;
    }
  }

  onMount(() => {
    userInfo().then((playerInfo) => {
      if (playerInfo !== null) {
        ownPlayerInfo = playerInfo as PlayerInfo;
        console.log('Own ID:', ownPlayerInfo.id);
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
  <h2>Player:</h2>
  {#each roomInfo.players as player}
    <p>{player.name}</p>
  {/each}
  {#if roomInfo.owner === ownPlayerInfo?.id}
    <button
      onclick={() => sendWebSocketMessage({ type: CommandType.StartGame })}
    >
      Start Game
    </button>
  {/if}
{:else if gameState}
  <div class="enemyPlayerContainer">
    {#if enemyPlayer().length > 0}
      {#each enemyPlayer() as player}
        <Player
          name={roomInfo.players.find((p) => p.id === player.id)?.name ||
            'Unknown Player'}
          playerGameInfo={player}
          cards={[undefined, undefined]}
          turn={gameState.current_player === player.id}
        />
      {/each}
    {/if}
  </div>

  <div class="centerCards">
    <div class="cardContainer">
      {#each paddedCards() as card}
        <div class="card">
          {card.rank} of {card.suit}
        </div>
      {/each}
    </div>
  </div>

  <div class="ownPlayerContainer">
    {#if ownPlayer() && ownPlayerInfo}
      <Player
        name={ownPlayerInfo?.name || 'You'}
        playerGameInfo={ownPlayer()!}
        cards={ownCards}
        turn={gameState?.current_player === ownPlayerInfo.id}
      />
    {/if}
  </div>

  <div class="actions">
    <button onclick={() => sendWebSocketMessage({ type: CommandType.Fold })}>
      Fold
    </button>
    <button onclick={() => sendWebSocketMessage({ type: CommandType.Call })}>
      Call
    </button>
    <button onclick={() => sendWebSocketMessage({ type: CommandType.Check })}>
      Check
    </button>

    <div class="raise-container">
      <input
        type="range"
        bind:value={raiseAmount}
        min="1"
        max="150"
        step="1"
        defaultValue="10"
      />
      <p>{raiseAmount}</p>
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
{:else}
  <p>Loading game information...</p>
{/if}

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

  .centerCards {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 5px;
    margin: 20px;
  }

  .actions {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: 30px;
    margin: 20px;
  }

  .raise-container {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 5px;
  }

  .raise-container p {
    width: 25px;
  }
</style>
