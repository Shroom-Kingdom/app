import { Router } from 'itty-router';

import { courseRouter } from './course';

export const router = Router();

router.all('/course/*', courseRouter.handle);

router.all('*', () => {
  console.log('404');
  return new Response('', { status: 404 });
});
