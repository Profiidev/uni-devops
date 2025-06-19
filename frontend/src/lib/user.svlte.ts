import { ResponseType, get } from '$lib/util.svelte';

export const user_info = async (userId?: string) => {
  const url = userId ? `/user/info/${userId}` : '/user/info';
  let result = await get<{}>(url, ResponseType.Json);

  if (typeof result === 'object' && result) {
    return result;
  }
  return null;
};
