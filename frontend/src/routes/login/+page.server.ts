import { redirect } from '@sveltejs/kit';

export const load = ({ cookies }) => {
  const token = cookies.get('token');
  if (token) {
    redirect(302, '/');
  }
};
