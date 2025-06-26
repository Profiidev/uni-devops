<script lang="ts">
  import type { Card } from '$lib/ws/message.svelte';

  export let card: Card | undefined;
</script>

<div class="card">
  {#if card !== undefined}
    <img
      src="/cards/{card.suit.toLowerCase()}_{card.rank.toLowerCase()}.svg"
      alt="{card.rank} of {card.suit}"
    />
  {:else}
    <img src="/cards/back.svg" alt="Back of Card" />
  {/if}
</div>

<style>
  .card {
    --card-width: clamp(60px, 10vw, 100px);
    width: var(--card-width);
    height: calc(1.4 * var(--card-width));
  }

  img {
    width: 100%;
    height: 100%;
  }
</style>
