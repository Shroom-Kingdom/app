import { Router } from 'itty-router';

import { authRouter } from './auth';
import { courseRouter } from './course';

export const router = Router();

router.all('/course/*', courseRouter.handle);
router.all('/auth/*', authRouter.handle);

router.all('*', () => {
  console.log('404');
  return new Response('', { status: 404 });
});
