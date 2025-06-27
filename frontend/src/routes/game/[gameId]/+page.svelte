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
    type GameEnded
  } from '$lib/ws/message.svelte';
  import { userInfo } from '$lib/user.svelte.js';
  import { CommandType } from '$lib/ws/command.svelte.js';
  import PlayerComponent from './playerComponent.svelte';
  import CardComponent from './cardComponent.svelte';

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
  let roundEnded: RoundEnded | null = $state(null);
  let gameEnded: GameEnded | null = $state(null);

  function handleIncoming(data: Message) {
    console.log('Received data:', data);

    switch (data.type) {
      case MessageType.RoomInfo:
        roomInfo = data;
        break;
      case MessageType.InvalidCommand:
        console.error('Invalid command received:', data);
        break;
      case MessageType.GameInfo:
        gameState = data;
        if (roomInfo) roomInfo.game_running = true;
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
        if (!gameState) break;
        gameState.current_player = data.id;
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
        roundEnded = data;
        pot = 0;
        gameState.current_player = 'null';
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
        break;
      case MessageType.BackToLobby:
        if (roomInfo) roomInfo.game_running = false;
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
    {#if ownPlayer() && ownPlayerInfo}
      <PlayerComponent
        name={ownPlayerInfo?.name || 'You'}
        playerGameInfo={ownPlayer()!}
        cards={ownCards}
        turn={gameState?.current_player === ownPlayerInfo.id}
        hand={roundEnded?.players.find((p) => p.id === ownPlayerInfo?.id)
          ?.best_hand || null}
        winnings={roundEnded?.players.find((p) => p.id === ownPlayerInfo?.id)
          ?.winnings || null}
        winner={gameEnded?.winner === ownPlayerInfo?.id}
      />
    {/if}
  </div>

  <div class="actions">
    {#if !roundEnded}
      {#if !ownPlayer()?.folded}
        <button
          onclick={() => sendWebSocketMessage({ type: CommandType.Fold })}
        >
          Fold
        </button>
        {#if callAmount > (ownPlayer()?.bet || 0)}
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

        <div class="raiseContainer">
          <input
            type="range"
            bind:value={raiseAmount}
            min="1"
            max={maxRaiseAmount()}
            step="1"
          />
          <input
            type="number"
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
      {/if}
    {:else if roomInfo?.owner === ownPlayerInfo?.id}
      {#if gameEnded}
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
    flex-direction: row;
    flex-wrap: wrap;
    align-items: center;
    justify-content: center;
    gap: 30px;
    margin: 20px;
  }

  .actions button {
    padding: 5px 10px;
    background-color: var(--primary-color);
    border: 3px solid var(--surface-background);
    border-radius: 10px;
    min-width: 80px;
    cursor: pointer;
    font-size: large;
  }

  .raiseContainer {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 10px;
  }
</style>
