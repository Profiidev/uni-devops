<script lang="ts">
  import type { Card } from '$lib/ws/message.svelte';

  export let card: Card | undefined;
</script>

<div class="card">
  {#if card !== undefined}
    {card.rank} of {card.suit}
  {:else}
    ?
  {/if}
</div>

<style>
  .card {
    padding: 8px;
    border: 2px solid var(--surface-background);
    color: var(--surface-background);
    border-radius: 15px;
    text-align: center;
    --card-width: clamp(60px, 10vw, 100px);
    width: var(--card-width);
    height: calc(1.4 * var(--card-width));
    background-color: var(--text-color);
  }
</style>
