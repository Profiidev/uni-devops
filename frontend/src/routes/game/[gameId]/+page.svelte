<script lang="ts">
  import { onMount } from 'svelte';
  import {
    connectWebSocket,
    closeWebSocket,
    sendWebSocketMessage
  } from '$lib/websocket.svelte';

  export let data;
  const { gameId } = data;

  let messages: string[] = [];

  function handleIncoming(data: any) {
    messages = [...messages, JSON.stringify(data)];
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

<button on:click={sendMessage}>Send Message</button>

<ul>
  {#each messages as message}
    <li>{message}</li>
  {/each}
</ul>
