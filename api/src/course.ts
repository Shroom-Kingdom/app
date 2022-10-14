import { Router } from 'itty-router';

const router = Router({ base: '/course' });
export { router as courseRouter };

router.get('/', async () => {
  return new Response('Hello World!');
});
