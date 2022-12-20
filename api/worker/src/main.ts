import { Router } from 'itty-router';

import { authRouter } from './auth';
import { courseRouter } from './course';

export const router = Router();

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
router.all!('/course/*', courseRouter.handle);
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
router.all!('/auth/*', authRouter.handle);

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
router.all!('*', () => {
  console.log('404');
  return new Response('', { status: 404 });
});
