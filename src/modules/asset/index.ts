import { Writable, writable } from 'svelte/store';

export interface AssetContext {
  data: Writable<Uint8Array | null>;
}

export const initialAssetState: AssetContext = {
  data: writable<Uint8Array | null>(null)
};

export const assetKey = {};
