import type { WalletSelector } from '@near-wallet-selector/core';
import { Unsubscriber, writable } from 'svelte/store';

import type { Account } from '../../../../common-types';

export const account$ = writable<Account | null>(null);
export const walletId$ = writable<string | null>(null);
export const isRegistered$ = writable<Promise<boolean>>(Promise.resolve(false));
export const accessToken$ = writable<string | null>(null);

export const selector$ = writable<WalletSelector | null>(null);

export interface WalletMetadata {
  url: string;
  extensionUrl?: string;
  twitter?: string;
  telegram?: string;
  discord?: string;
  disabled?: boolean;
}

export const WALLETS: Record<string, WalletMetadata> = {
  'near-wallet': {
    url: 'https://wallet.near.org'
  },
  sender: {
    url: 'https://sender.org/',
    twitter: 'https://twitter.com/SenderWallet',
    disabled: true
  },
  'meteor-wallet': {
    url: 'https://meteorwallet.app/',
    extensionUrl:
      'https://chrome.google.com/webstore/detail/meteor-wallet/pcndjhkinnkaohffealmlmhaepkpmgkb',
    twitter: 'https://twitter.com/MeteorWallet',
    disabled: true
  }
};

export async function fetchWithAuth(
  input: RequestInfo,
  init?: RequestInit<RequestInitCfProperties> | undefined
) {
  let unsubscribe: Unsubscriber;
  const accessToken = await new Promise<string>(resolve => {
    unsubscribe = accessToken$.subscribe(tkn => {
      if (!tkn) return;
      resolve(tkn);
    });
  });
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  //@ts-ignore
  unsubscribe();
  const walletId = await new Promise<string>(resolve => {
    unsubscribe = walletId$.subscribe(wid => {
      if (!wid) return;
      resolve(wid);
    });
  });
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  //@ts-ignore
  unsubscribe();
  return fetch(input, {
    ...init,
    headers: { ...init?.headers, TOKEN: accessToken, WALLET_ID: walletId }
  });
}
