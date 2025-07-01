import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
  const { gameId } = params;

  console.log('Game ID:', gameId);

  return {
    gameId
  };
};
