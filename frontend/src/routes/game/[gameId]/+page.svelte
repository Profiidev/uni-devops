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
    type GameInfo,
    type Card,
    type RoundEnded,
    type PlayerGameInfo
  } from '$lib/ws/message.svelte.js';
  import { user_info } from '$lib/user.svelte.js';
  import { CommandType } from '$lib/ws/command.svelte.js';
  import Player from './player.svelte';

  const { data } = $props();
  const { gameId } = data;

  // let messages: string[] = [];

  let roomInfo: RoomInfo = $state({
    type: MessageType.RoomInfo,
    id: gameId,
    players: [],
    owner: '',
    game_running: false,
    starting_cash: 0,
    small_blind: 0,
    created_at: new Date().toISOString()
  });
  let ownPlayerInfo: PlayerInfo | undefined = $state();
  let gameInfo: GameInfo | undefined = $state();
  let ownPlayer: () => PlayerGameInfo | undefined = $derived(() => {
    return gameInfo?.players.find((p) => p.id === ownPlayerInfo!.id);
  });
  let enemyPlayer: () => PlayerGameInfo[] | undefined = $derived(() => {
    const ps = gameInfo?.players || [];
    const index = ps.findIndex((p) => p.id === ownPlayerInfo!.id);
    if (index === -1) return undefined;
    return [...ps.slice(index + 1), ...ps.slice(0, index)];
  });
  let ownCards: Card[] = $state([]);
  let raiseAmount: number = $state(0);
  let revealedCards: Card[] = $state([]);
  let paddedCards = $derived(() => 
    Array.from({ length: 5 }, (_, i) => revealedCards[i] || { rank: '?', suit: '?' })
  );
  let callAmount: number = 0;
  let roundEnded: RoundEnded | undefined = $state();

  function handleIncoming(data: any) {
    // messages = [...messages, JSON.stringify(data)];
    console.log('Received data:', data);

    switch (data.type) {
      case 'RoomInfo':
        roomInfo = data;
        callAmount = data.small_blind * 2;
        console.log(data.small_blind, callAmount);
        break;
      case 'InvalidCommand':
        console.error('Invalid command received:', data);
        break;
      case 'GameInfo':
        gameInfo = data;
        roomInfo.game_running = true;
        break;
      case 'GameStarted':
        roomInfo.game_running = true;
        break;
      case 'PlayerJoined':
        if (!roomInfo.players.some((p) => p.id === data.id)) {
          roomInfo.players = [
            ...(roomInfo.players || []),
            { id: data.id, name: data.name }
          ];
        }
        break;
      case 'PlayerLeft':
        roomInfo.players = roomInfo.players.filter(
          (p: PlayerInfo) => p.id !== data.id
        );
        break;
      case 'PlayerTurn':
        if (gameInfo) gameInfo.current_player = data.id;
        else sendWebSocketMessage({ type: CommandType.GameInfo });
        break;
      case 'YourCards':
        ownCards = data.cards;
        break;
      case 'CardRevealed':
        revealedCards = [
          ...revealedCards,
          { rank: data.rank, suit: data.suit }
        ];
        console.log('Revealed cards:', revealedCards);
        break;
      case 'PlayerActed':
        switch (data.action.type) {
          case 'Fold':
            if (gameInfo) {
              gameInfo.players.find((p) => p.id === data.id)!.folded = true;
            }
            break;
          case 'Call':
            if (gameInfo) {
              const player = gameInfo.players.find((p) => p.id === data.id);
              if (player) {
                console.log(
                  callAmount,
                  player.bet,
                  player.cash,
                  callAmount - player.bet
                );
                const difference = Math.min(
                  callAmount - player.bet,
                  player.cash
                );
                player.bet += difference;
                player.cash -= difference;
              }
            }
            break;
          case 'Check':
            break;
          case 'Raise':
            if (gameInfo) {
              const player = gameInfo.players.find((p) => p.id === data.id);
              if (player) {
                callAmount += data.action.amount;
                player.bet += data.action.amount;
                player.cash -= data.action.amount;
              }
            }
            break;
        }
        break;
    }
  }

  onMount(() => {
    user_info().then((playerInfo) => {
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

{#if !roomInfo.game_running}
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
{:else if gameInfo}
  <div class="enemyPlayerContainer">
    {#if enemyPlayer !== undefined}
      {#each enemyPlayer()! as player}
        <Player
          name={roomInfo.players.find((p) => p.id === player.id)?.name ||
            'Unknown Player'}
          playerGameInfo={player}
          cards={[undefined, undefined]}
          turn={gameInfo.current_player === player.id}
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
    {#if ownPlayer !== undefined}
      <Player
        name={ownPlayerInfo?.name || 'You'}
        playerGameInfo={ownPlayer()!}
        cards={ownCards}
        turn={gameInfo.current_player === ownPlayerInfo?.id}
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
