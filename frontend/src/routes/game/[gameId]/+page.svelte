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
    type PlayerInfo
  } from '$lib/ws/message.svelte.js';
  import { user_info } from '$lib/user.svelte.js';

  const { data } = $props();
  const { gameId } = data;

  // let messages: string[] = [];

  let roomInfo: RoomInfo = $state({
    type: MessageType.RoomInfo,
    id: gameId,
    players: [],
    owner: '',
    gameRunning: false,
    startingCash: 0,
    smallBlind: 0,
    created_at: new Date().toISOString()
  });
  let ownId: string | undefined;

  onMount(() => {
    user_info().then((id) => {
      ownId = typeof id === 'string' ? id : undefined;
    });
    connectWebSocket(gameId, handleIncoming);

    return () => {
      closeWebSocket();
    };
  });

  function handleIncoming(data: any) {
    // messages = [...messages, JSON.stringify(data)];
    console.log('Received data:', data);

    switch (data.type) {
      case 'RoomInfo':
        roomInfo = data;
        break;
      case 'InvalidCommand':
        break;
      case 'GameInfo':
        break;
      case 'GameStarted':
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
        break;
      case 'YourCards':
        break;
      case 'CardRevealed':
        break;
      case 'PlayerActed':
        break;
    }
  }

  onMount(() => {
    connectWebSocket(gameId, handleIncoming);

    return () => {
      closeWebSocket();
    };
  });

  function sendMessage() {
    sendWebSocketMessage({ text: 'Hello from Svelte!' });
  }
</script>

<h1>Game ID: {gameId}</h1>

<button onclick={sendMessage}>Send Message</button>

<h2>Player:</h2>
{#if !roomInfo.gameRunning}
  <p>
    {roomInfo.players.map((p) => p.name).join(', ')}
  </p>
{/if}

<ul>
  <!-- {#each messages as message}
    <li>{message}</li>
  {/each} -->
</ul>
