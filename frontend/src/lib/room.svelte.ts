import {
  ContentType,
  ResponseType,
  get,
  post,
  del
} from '$lib/util.svelte';

export const create_Room = async () => {
  let result = await post<{ id: string }>(
    '/game/room/create',
    ResponseType.Json,
    ContentType.Json,
    undefined
  );

  if (typeof result === 'object' && result && 'id' in result) {
    return result.id;
  }
  return null;
};

export const list_Rooms = async () => {
  let rooms = await get<{ rooms: string[] }>(
    '/game/room/list',
    ResponseType.Json
  );

  if (typeof rooms === 'object') {
    return rooms;
  }
  return null;
};

export const delete_Room = async (id: string) => {
  await del<undefined>(
    `/game/room/${id}`,
    ResponseType.None
  );
};
