<script lang="ts">
  import type { Card, PlayerGameInfo } from '$lib/ws/message.svelte';
  import CardComponent from './cardComponent.svelte';

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
  <div class="cardContainer">
    {#each cards as card}
      <div class="cardWrapper">
        <CardComponent {card} />
      </div>
    {/each}
  </div>
  <div class="info">
    <h1 class="name">{name}</h1>
    <p class="cash">${playerGameInfo.cash}</p>
  </div>
</div>

<style>
  .player {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    width: fit-content;
    margin: clamp(45px, 11vw, 110px) 20px -10px 20px;
    font-size: large;
    padding: 30px;
    z-index: -1;
    border-radius: 50%;
    border: 2px dashed var(--background-color);
  }

  .playerTurn {
    border: 2px dashed var(--primary-color);
  }

  .info {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    z-index: 1;
  }

  .name {
    width: clamp(90px, 13vw, 130px);
    background-color: var(--primary-color);
    border: 3px solid var(--surface-background);
    text-align: center;
  }

  .cash {
    width: clamp(65px, 9vw, 90px);
    background-color: var(--text-color);
    border: 3px solid var(--surface-background);
    color: var(--surface-background);
    text-align: center;
    border-top: none;
  }

  .folded {
    opacity: 0.5;
  }

  .cardContainer {
    position: absolute;
    display: flex;
    gap: -20px;
    z-index: 0;
    transform: translateY(calc(-1 * clamp(42px, 7.4vw, 74px)));
  }

  .cardWrapper {
    transform-origin: center;
  }

  .cardContainer > *:first-child {
    transform: rotate(-20deg) translateX(clamp(12.5px, 2.5vw, 25px));
  }

  .cardContainer > *:nth-child(2) {
    transform: rotate(20deg) translateX(calc(-1 * clamp(12.5px, 2.5vw, 25px)));
  }
</style>
