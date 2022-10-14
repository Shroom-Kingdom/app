import { Router } from 'itty-router';

export class User {
  private state: DurableObjectState;
  private env: Env;
  private initializePromise: Promise<void> | undefined;
  private router: Router<unknown>;

  constructor(state: DurableObjectState, env: Env) {
    this.state = state;
    this.env = env;
    this.router = Router()
      .post('/upload', async () => {
        return new Response('', { status: 501 });
      })
      .post('/publish', async () => {
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
