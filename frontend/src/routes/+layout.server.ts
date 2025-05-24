import { redirect } from '@sveltejs/kit';

export const load = async ({ cookies, url }) => {
  const cookie = cookies.get('token');

  if (url.pathname !== '/login') {
    if (!cookie) {
      throw redirect(302, '/login');
    }
  } else {
    if (cookie) {
      throw redirect(302, '/');
    }
  }
};
