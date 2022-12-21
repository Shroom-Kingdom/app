import { type IRequest, Router, type RouterType } from 'itty-router';

export async function setSession(
  accountId: string,
  accessToken: string,
  env: Env
) {
  const addr = env.SESSIONS.idFromName(accountId);
  const obj = env.SESSIONS.get(addr);
  await obj.fetch('http://session.com', {
    method: 'POST',
    body: accessToken
  });
}

export async function validateSession(
  accountId: string,
  accessToken: string,
  env: Env
) {
  const addr = env.SESSIONS.idFromName(accountId);
  const obj = env.SESSIONS.get(addr);
  await obj.fetch(`http://session.com?access_token=${accessToken}`);
}

export class Session {
  private state: DurableObjectState;
  private initializePromise: Promise<void> | undefined;
  private router: RouterType;
  private accessToken?: string;

  constructor(state: DurableObjectState) {
    this.state = state;
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    this.router = Router().post!('*', async (req: IRequest) => {
      const accessToken = await req.text();
      this.accessToken = accessToken;
      this.state.storage.put('accessToken', accessToken);
      return new Response('', { status: 204 });
    }).get!('*', async (req: IRequest) => {
      const accessToken = req.query.access_token as string;
      if (accessToken === this.accessToken) {
        return new Response('', { status: 204 });
      } else {
        return new Response('', { status: 401 });
      }
    });
  }

  async initialize(): Promise<void> {
    this.accessToken = await this.state.storage.get('accessToken');
  }

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
