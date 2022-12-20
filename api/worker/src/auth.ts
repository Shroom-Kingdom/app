import { Sha256 } from '@aws-crypto/sha256-browser';
import {
  keyStores,
  connect,
  ConnectConfig,
  utils
} from '@tarnadas/near-api-js';
import { type IRequest, Router } from 'itty-router';
import { sign } from 'tweetnacl';

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
router.post!('/login', async (req: IRequest) => {
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
  const account = await near.account(accountId);
  const accessKeys = await account.getAccessKeys();
  console.log('accessKeys', accessKeys);

  const publicKeys = accessKeys.map(key =>
    utils.PublicKey.fromString(key.public_key)
  );
  console.log('publicKeys', publicKeys);
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
  console.log('hasPubKey', hasPubKey);
  return new Response('', { status: 501 });
});
