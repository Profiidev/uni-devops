<script lang="ts">
  import { create_Room, delete_Room, list_Rooms } from '$lib/room.svelte';
  import { user_info } from '$lib/user.svlte';
  import { onMount } from 'svelte';

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
    updateRooms();
  };

  const deleteRoomClicked = async (roomId: string) => {
    await delete_Room(roomId);
    console.log('Deleted room with ID:', roomId);
    updateRooms();
  };

  const getUserInfo = async () => {
    user = ((await user_info()) as User) ?? null;
  };

  onMount(() => {
    updateRooms();
    getUserInfo();
  });
</script>

<main>
  <h1>Dealing With DevOps</h1>
  <h2>Rooms</h2>
  <button onclick={createRoomClicked}>create new Room</button>

  {#each rooms as room, i}
    <div class="room" id={room.id}>
      <h3>Your {i + 1}. Room</h3>
      <div class="room-details">
        <p>Created At: {room.created_at}</p>
        <p>Game Running: {room.game_running ? 'Yes' : 'No'}</p>
        <p>Starting Cash: {room.starting_cash}</p>
        <p>Small Blind: {room.small_blind}</p>
        <button onclick={() => deleteRoomClicked(room.id)}>delete</button>
      </div>
    </div>
  {/each}
</main>

<style>
  .room {
    width: fit-content;
  }
  .room h3 {
    cursor: pointer;
  }
  .room:hover .room-details {
    opacity: 1;
    display: block;
  }
  .room-details {
    position: absolute;
    display: none;
    opacity: 0;
    border: 1px solid var(--text-color);
    background-color: var(--surface-background);
    padding: 8px;
    margin-left: 20px;
  }
  .room-details button {
    cursor: pointer;
  }
</style>
