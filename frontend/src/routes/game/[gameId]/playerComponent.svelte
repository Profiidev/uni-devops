<script lang="ts">
  import {
    PlayerActionType,
    type Card,
    type Hand,
    type PlayerAction,
    type PlayerGameInfo
  } from '$lib/ws/message.svelte';
  import CardComponent from './cardComponent.svelte';

  const {
    name,
    playerGameInfo,
    cards,
    turn,
    hand,
    winnings,
    winner,
    lastAction,
    roundRunning
  } = $props<{
    name: String;
    playerGameInfo: PlayerGameInfo;
    cards: Card[] | undefined[];
    turn: boolean;
    hand: Hand | null;
    winnings: number | null;
    winner: boolean;
    lastAction: PlayerAction | null;
    roundRunning: boolean;
  }>();

  const endInfo: () => String = $derived(() => {
    let info = '';
    if (hand) {
      info = hand.type.replace(/([A-Z])/g, ' $1').trim();
      if (winnings) {
        info += `: +$${winnings}`;
      }
    } else if (winnings) {
      info = `+$${winnings}`;
    }
    return info;
  });
</script>

<div
  class="player {turn && roundRunning ? 'playerTurn' : ''} {winner
    ? 'gameWon'
    : ''} {playerGameInfo?.folded ? 'folded' : ''}"
>
  {#if !playerGameInfo?.folded}
    <div class="cardContainer">
      {#each cards as card}
        <div class="cardWrapper">
          <CardComponent {card} />
        </div>
      {/each}
    </div>
  {:else}
    <p class="fold">Fold</p>
  {/if}
  <div class="playerDetails">
    <h1 class="name">{name}</h1>
    <p class="cash">${playerGameInfo.cash}</p>
  </div>
  {#if endInfo()}
    <p class="speachBubble {winnings ? 'winner' : ''}">
      {endInfo()}
    </p>
  {:else if lastAction}
    <p class="speachBubble">
      {lastAction.type}
      {#if lastAction.type === PlayerActionType.Raise}
        ${lastAction.amount}
      {/if}
    </p>
  {/if}
  <div class="sideDisplay">
    <p>
      {#if playerGameInfo.bet > 0 && roundRunning}
        Bet: ${playerGameInfo.bet}
      {/if}
    </p>
  </div>
</div>

<style>
  .player {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    width: fit-content;
    margin: clamp(45px, 11vw, 110px) 20px 20px 20px;
    font-size: large;
    padding: 30px;
    z-index: -1;
    border-radius: 50%;
    border: 2px dashed var(--background-color);
  }

  .playerTurn {
    border: 2px dashed var(--primary-color);
  }

  .gameWon {
    outline: 2px solid gold;
    border: none;
    outline-offset: 30px;
    .name {
      background-color: gold;
      color: var(--surface-background);
    }
  }

  .playerDetails {
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

  .cardContainer > div:first-child {
    transform: rotate(-20deg) translateX(clamp(12.5px, 2.5vw, 25px));
  }

  .cardContainer > div:nth-child(2) {
    transform: rotate(20deg) translateX(calc(-1 * clamp(12.5px, 2.5vw, 25px)));
  }

  .fold {
    position: absolute;
    z-index: 2;
    transform: translateY(-60px);
    font-weight: bold;
    font-size: x-large;
  }

  .winner {
    color: var(--primary-color);
  }

  .speachBubble {
    position: absolute;
    font-size: large;
    line-height: 24px;
    min-width: 100px;
    background: var(--text-color);
    border-radius: 40px;
    padding: 4px;
    text-align: center;
    color: var(--surface-background);
    margin-top: 40px;
    transform: translateY(55px);
  }

  .speachBubble::before {
    content: '';
    width: 0px;
    height: 0px;
    position: absolute;
    border-left: 24px solid var(--text-color);
    border-right: 12px solid transparent;
    border-bottom: 12px solid var(--text-color);
    border-top: 20px solid transparent;
    left: 32px;
    top: -24px;
  }

  .sideDisplay {
    position: absolute;
    transform: translateX(clamp(95px, 15vw, 150px)) translateY(-15px);
    width: 90px;
  }
</style>
