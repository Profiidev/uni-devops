import { redirect } from '@sveltejs/kit';

export const load = async ({ cookies, url }) => {
  const cookie = cookies.get('token');

  if (url.pathname !== '/login') {
    if (!cookie) {
      throw redirect(
        302,
        `/login?redirectTo=${encodeURIComponent(url.pathname)}`
      );
    }
  } else {
    if (cookie) {
      const redirectTo = url.searchParams.get('redirectTo') || '/';
      throw redirect(302, redirectTo);
    }
  }
};
