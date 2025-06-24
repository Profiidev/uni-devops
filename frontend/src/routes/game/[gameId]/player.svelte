<script lang="ts">
  import type { Card, PlayerGameInfo } from '$lib/ws/message.svelte';

  export let name: String;
  export let playerGameInfo: PlayerGameInfo;
  export let cards: Card[] | undefined[];
  export let turn: boolean;
</script>

<div
  class="player {turn ? 'playerTurn' : ''} {playerGameInfo?.folded
    ? 'folded'
    : ''}"
>
  <h1>
    {name}
  </h1>
  <p>Cash: {playerGameInfo.cash}</p>
  <p>Bet: {playerGameInfo.bet}</p>
  <div class="cardContainer">
    {#each cards as card}
      <div class="card">
        {#if card !== undefined}
          {card.rank} of {card.suit}
        {:else}
          ?
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .player {
    border: 1px solid #ccc;
    border-radius: 4px;
    width: fit-content;
  }

  .playerTurn {
    border-color: green;
  }

  .folded {
    opacity: 0.5;
  }

  .cardContainer {
    display: flex;
    gap: 8px;
  }

  .card {
    padding: 8px;
    border: 1px solid white;
    border-radius: 4px;
    min-width: 60px;
    text-align: center;
  }
</style>
