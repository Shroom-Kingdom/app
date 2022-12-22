import { Sha256 } from '@aws-crypto/sha256-browser';
import {
  keyStores,
  connect,
  ConnectConfig,
  utils
} from '@tarnadas/near-api-js';
import { type IRequest, Router } from 'itty-router';
import { sign } from 'tweetnacl';
import { v4 as uuidv4 } from 'uuid';

import { Account, NearRegister } from '../../../common-types';

import { setSession } from './session';
import { getAccount, registerAccountViaNear } from './user';

const router = Router({ base: '/auth' });
export { router as authRouter };

const TOKEN_EXPIRY_TIME = 1_000 * 10;
const nearConfig: ConnectConfig = {
  networkId: 'testnet',
  keyStore: new keyStores.InMemoryKeyStore(),
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org'
};

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
router.post!('/login', async (req: IRequest, env: Env) => {
  const accessToken = await req.text();
  if (!accessToken) {
    return new Response('', { status: 400 });
  }
  const tokenparts = accessToken.split('.');
  const msgBytes = tokenparts[0];
  const signature = atob(tokenparts[1]);
  const { accountId, iat } = JSON.parse(atob(msgBytes)) as {
    accountId?: string;
    iat?: number;
  };
  if (accountId == null || iat == null) {
    return new Response('', { status: 400 });
  }
  if (
    iat + TOKEN_EXPIRY_TIME < new Date().getTime() ||
    iat - 1000 > new Date().getTime()
  ) {
    console.error(`token issued at ${iat} has expired for ${accountId}`);
    return new Response('', { status: 400 });
  }

  const near = await connect(nearConfig);
  const nearAccount = await near.account(accountId);
  const accessKeys = await nearAccount.getAccessKeys();

  const publicKeys = accessKeys.map(key =>
    utils.PublicKey.fromString(key.public_key)
  );
  const encoder = new TextEncoder();
  const hasPubKey = !!publicKeys.find(async pk => {
    const hash = new Sha256();
    hash.update(msgBytes);
    const sha256 = await hash.digest();
    return sign.detached.verify(
      new Uint8Array(sha256),
      new Uint8Array(encoder.encode(signature)),
      new Uint8Array(pk.data)
    );
  });
  if (!hasPubKey) {
    return new Response('', { status: 401 });
  }

  await setSession(accountId, accessToken, env);
  const account = await getAccount(accountId, env);

  if (account == null) {
    return new Response('', { status: 204 });
  }
  return new Response(JSON.stringify(account));
}).post!('/register/near', async (req: IRequest, env: Env) => {
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
  const isOk = await registerAccountViaNear(account, env);
  if (!isOk) {
    return new Response('', { status: 500 });
  }
  return new Response(JSON.stringify(account));
}).post!('/register/discord', async () => {
  return new Response('', { status: 501 });
});
