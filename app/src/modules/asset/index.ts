import { writable } from 'svelte/store';

export const assets = writable<Uint8Array | null>(null);
