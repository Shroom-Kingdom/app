import { Router } from 'itty-router';

import { logErrorResponse } from './helpers';
import { isCourse } from './wasm/shrm_api_wasm';

const router = Router({ base: '/course' });
export { router as courseRouter };

router
  .post('/check', async (req: Request) => {
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
  })
  .post('/upload', async (req: Request, env: Env) => {
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
  })
  .post('/publish', async () => {
    return new Response('', { status: 501 });
  });

export class Courses {
  private state: DurableObjectState;
  private env: Env;
  private initializePromise: Promise<void> | undefined;
  private router: Router<unknown>;

  constructor(state: DurableObjectState, env: Env) {
    this.state = state;
    this.env = env;
    this.router = Router().post('/publish', async () => {
      return new Response('', { status: 501 });
    });
  }

  // eslint-disable-next-line @typescript-eslint/no-empty-function
  async initialize(): Promise<void> {}

  async fetch(request: Request): Promise<Response> {
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
