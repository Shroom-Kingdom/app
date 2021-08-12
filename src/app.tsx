import React, { FC, useContext } from 'react';

import { AssetContext, AssetLoader } from './modules/asset';
import { Debug } from './modules/debug';

export const App: FC = () => {
  const { assetState } = useContext(AssetContext);
  return (
    <>
      <div className="app">
        <style jsx>{`
          .app {
            position: relative;
            display: grid;
          }
        `}</style>
        {assetState.data ? <Debug /> : <AssetLoader />}
      </div>
      <canvas id="canvas">
        <style jsx>{`
          #canvas {
            position: absolute;
            top: 0;
            left: 0;
            z-index: -1;
            width: 1280px;
            height: 720px;
          }
        `}</style>
      </canvas>
    </>
  );
};
