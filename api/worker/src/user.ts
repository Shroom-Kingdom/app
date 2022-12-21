import { type IRequest, Router, type RouterType } from 'itty-router';
import { v4 as uuidv4 } from 'uuid';

import type { Account } from '../../../common-types';

import { NearRegister } from './requests';

export async function getAccount(
  accountId: string,
  env: Env
): Promise<Account | undefined> {
  const addr = env.SESSIONS.idFromName(accountId);
  const obj = env.SESSIONS.get(addr);
  const res = await obj.fetch('http://session.com/account');
  if (res.ok) {
    return res.json<Account>();
  }
}
export class User {
  private state: DurableObjectState;
  private initializePromise: Promise<void> | undefined;
  private router: RouterType;
  private account?: Account;

  constructor(state: DurableObjectState) {
    this.state = state;
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    this.router = Router().post!('/upload', async () => {
      return new Response('', { status: 501 });
    }).post!('/publish', async () => {
      return new Response('', { status: 501 });
    }).get!('/account', async () => {
      if (!this.account) {
        return new Response('', { status: 500 });
      }
      return new Response(JSON.stringify(this.account));
    }).post!('/register/near', async (req: IRequest) => {
      // TODO zod validation
      const { username, walletId } = await (
        req as unknown as Request
      ).json<NearRegister>();
      const uuid = uuidv4();
      // TODO check unique uuid
      const account: Account = {
        uuid,
        username,
        walletId
      };
      const accountJson = JSON.stringify(account);
      await this.state.storage.put('account', accountJson);
      return new Response(accountJson);
    }).post!('/register/discord', async () => {
      return new Response('', { status: 501 });
    });
  }

  // eslint-disable-next-line @typescript-eslint/no-empty-function
  async initialize(): Promise<void> {
    this.account = await this.state.storage.get<Account>('account');
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
