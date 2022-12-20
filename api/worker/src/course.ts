import { type IRequest, Router, type RouterType } from 'itty-router';

import { logErrorResponse } from './helpers';
import { isCourse } from './wasm/shrm_api_wasm';

const router = Router({ base: '/course' });
export { router as courseRouter };

/* eslint-disable @typescript-eslint/no-non-null-assertion */
router.post!('/check', async (req: IRequest) => {
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
}).post!('/upload', async (req: IRequest, env: Env) => {
  if (!req.arrayBuffer) {
    return new Response('', { status: 500 });
  }
  const arrayBuffer = await req.arrayBuffer();
  const buffer = new Uint8Array(arrayBuffer);
  const isOk = isCourse(buffer);
  if (!isOk) {
    return new Response('', { status: 400 });
  }
  // TODO id from session
  const id = '';
  const addr = env.USER.idFromName(id);
  const obj = env.USER.get(addr);
  const res = await obj.fetch(req.url, {
    method: 'POST',
    body: arrayBuffer
  });
  if (!res.ok) {
    logErrorResponse('[POST] /course/upload', res);
    return new Response('', { status: 400 });
  }
  return new Response('', { status: 204 });
}).post!('/publish', async () => {
  return new Response('', { status: 501 });
});
/* eslint-enable @typescript-eslint/no-non-null-assertion */

export class Courses {
  private state: DurableObjectState;
  private env: Env;
  private initializePromise: Promise<void> | undefined;
  private router: RouterType;

  constructor(state: DurableObjectState, env: Env) {
    this.state = state;
    this.env = env;
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    this.router = Router().post!('/publish', async () => {
      return new Response('', { status: 501 });
    });
  }

  // eslint-disable-next-line @typescript-eslint/no-empty-function
  async initialize(): Promise<void> {}

  async fetch(request: IRequest): Promise<Response> {
    if (!this.initializePromise) {
      this.initializePromise = this.initialize().catch(err => {
        this.initializePromise = undefined;
        throw err;
      });
    }
    await this.initializePromise;

    return this.router.handle(request);
  }
}
