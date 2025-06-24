<script lang="ts">
  import { create_Room, delete_Room, list_Rooms } from '$lib/room.svelte';
  import { userInfo } from '$lib/user.svelte';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  type User = {
    id: string;
    name: string;
    email: string;
    image: string | null;
  };

  let user: User | null = $state(null);

  type Room = {
    id: string;
    owner: string;
    created_at: string;
    game_running: boolean;
    starting_cash: number;
    small_blind: number;
  };

  let rooms: Room[] = $state([]);

  const updateRooms = async () => {
    const result = await list_Rooms();
    if (result && Array.isArray(result.rooms)) {
      rooms = result.rooms as unknown as Room[];
    } else if (Array.isArray(result)) {
      rooms = result as Room[];
    } else {
      rooms = [];
    }
  };

  const createRoomClicked = async () => {
    let id = await create_Room();
    console.log('Created room with ID:', id);
    goto(`/game/${id}`);
  };

  const deleteRoomClicked = async (roomId: string) => {
    await delete_Room(roomId);
    console.log('Deleted room with ID:', roomId);
    updateRooms();
  };

  const getUserInfo = async () => {
    user = ((await userInfo()) as User) ?? null;
  };

  onMount(() => {
    updateRooms();
    getUserInfo();
  });

  const displayDate = (dateString: string) => {
    const date = new Date(dateString);
    let referenceDay = new Date();
    let day = date.toLocaleDateString([], {
      month: '2-digit',
      day: '2-digit',
      year: '2-digit'
    });
    if (
      date.getFullYear() === referenceDay.getFullYear() &&
      date.getMonth() === referenceDay.getMonth() &&
      date.getDate() === referenceDay.getDate()
    )
      day = 'Today';
    referenceDay.setDate(referenceDay.getDate() - 1);
    if (
      date.getFullYear() === referenceDay.getFullYear() &&
      date.getMonth() === referenceDay.getMonth() &&
      date.getDate() === referenceDay.getDate()
    )
      day = 'Yesterday';
    return (
      day +
      ' ' +
      date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
    );
  };
</script>

<main>
  <h1>Dealing With DevOps</h1>
  <div class="middle">
    <button class="primary-button" onclick={createRoomClicked}>New Game</button>
    <h2>Your Games:</h2>

    <div class="rooms">
      {#each rooms as room, i}
        <div class="room" id={room.id}>
          <p>{displayDate(room.created_at)}</p>
          <p>Running: {room.game_running ? 'Yes' : 'No'}</p>
          <p>Starting Cash: {room.starting_cash}</p>
          <p>Small Blind: {room.small_blind}</p>
          <div class="row">
            <button
              class="primary-button"
              onclick={() => goto(`/game/${room.id}`)}>join</button
            >
            <button onclick={() => deleteRoomClicked(room.id)}>delete</button>
          </div>
        </div>
      {/each}
    </div>
  </div>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: 2rem;
    gap: 2rem;
  }
  .middle {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    border-radius: 15px;
    padding: 3.5rem;
    gap: 1rem;
    background-color: var(--surface-background);
    width: min(90vw, 700px);
  }
  h1 {
    font-size: 2rem;
  }
  h2 {
    font-size: 1.5rem;
  }
  p {
    font-size: 1rem;
  }
  .primary-button {
    padding: 0.5rem 1rem;
    width: fit-content;
    background-color: var(--primary-color);
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  .rooms {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    width: 100%;
    flex-wrap: wrap;
  }
  .room {
    border: 2px solid var(--primary-color);
    border-radius: 8px;
    padding: 1rem;
  }
  .row {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    gap: 0.5rem;
  }
</style>
