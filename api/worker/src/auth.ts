import { Sha256 } from '@aws-crypto/sha256-browser';
import {
  keyStores,
  connect,
  ConnectConfig,
  utils
} from '@tarnadas/near-api-js';
import { type IRequest, Router } from 'itty-router';
import { sign } from 'tweetnacl';

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
  const signature = new Uint8Array(
    atob(tokenparts[1])
      .split(',')
      .map(c => Number(c))
  );
  const publicKey = utils.PublicKey.fromString(tokenparts[2]);
  const { walletId, iat } = JSON.parse(atob(msgBytes)) as {
    walletId?: string;
    iat?: number;
  };
  if (walletId == null || iat == null) {
    return new Response('', { status: 400 });
  }
  if (
    iat + TOKEN_EXPIRY_TIME < new Date().getTime() ||
    iat - TOKEN_EXPIRY_TIME > new Date().getTime()
  ) {
    console.error(
      `token issued at ${new Date(iat)} has expired for ${walletId}`
    );
    return new Response('', { status: 400 });
  }

  const hash = new Sha256();
  hash.update(msgBytes);
  const sha256 = await hash.digest();
  const verified = sign.detached.verify(
    sha256,
    new Uint8Array(signature),
    publicKey.data
  );
  if (!verified) {
    return new Response('', { status: 401 });
  }

  const near = await connect(nearConfig);
  const nearAccount = await near.account(walletId);
  const accessKeys = await nearAccount.getAccessKeys();

  const publicKeys = accessKeys.map(key =>
    utils.PublicKey.fromString(key.public_key)
  );
  const hasPubKey = !!publicKeys.find(
    pk => pk.toString() === publicKey.toString()
  );
  if (!hasPubKey) {
    return new Response('', { status: 401 });
  }

  await setSession(walletId, accessToken, env);
  const account = await getAccount(walletId, env);

  if (account == null) {
    return new Response('', { status: 204 });
  }
  return new Response(JSON.stringify(account), {
    headers: { 'content-type': 'application/json' }
  });
}).post!('/register/near', async (req: IRequest, env: Env) => {
  // TODO zod validation
  const { username, walletId } = await (
    req as unknown as Request
  ).json<NearRegister>();
  const account: Account = {
    username,
    walletId
  };
  const status = await registerAccountViaNear(account, env);
  if (status >= 400) {
    return new Response('', { status });
  }
  return new Response(JSON.stringify(account), {
    headers: { 'content-type': 'application/json' }
  });
}).post!('/register/discord', async () => {
  return new Response('', { status: 501 });
});
