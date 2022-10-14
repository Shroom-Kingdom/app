import { Router } from 'itty-router';

import { isCourse } from './wasm/shrm_api_wasm';

const router = Router({ base: '/course' });
export { router as courseRouter };

router.post('/check', async (req: Request) => {
  if (!req.arrayBuffer) {
    return new Response('', { status: 500 });
  }
  const arrayBuffer = await req.arrayBuffer();
  const buffer = new Uint8Array(arrayBuffer);
  const res = isCourse(buffer);
  if (res) {
    return new Response('', { status: 204 });
  } else {
    return new Response('', { status: 400 });
  }
});
