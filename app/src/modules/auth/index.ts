import { writable } from 'svelte/store';

import type { Account } from '../../../../common-types';

export const account = writable<Account | null>(null);
export const isRegistered = writable<boolean>(false);
