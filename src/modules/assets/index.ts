import { Dispatch, SetStateAction, createContext } from 'react';

export * from './asset-loader';

export interface AssetState {
  data?: Uint8Array;
}

export interface AssetContext {
  assetState: AssetState;
  setAssetState: Dispatch<SetStateAction<AssetState>>;
}

export const initialAssetState: AssetState = { data: undefined };

export const AssetContext = createContext<AssetContext>({
  assetState: initialAssetState,
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  setAssetState: () => {}
});
