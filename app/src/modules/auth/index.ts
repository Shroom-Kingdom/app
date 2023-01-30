import type { WalletSelector } from '@near-wallet-selector/core';
import { writable } from 'svelte/store';

import type { Account } from '../../../../common-types';

export const account$ = writable<Account | null>(null);
export const walletId$ = writable<string | null>(null);
// eslint-disable-next-line @typescript-eslint/no-empty-function
export const isRegistered$ = writable<Promise<boolean>>(new Promise(() => {}));
export const afterRegister$ = writable<number>(0);
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
