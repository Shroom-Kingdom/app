import React, { FC } from 'react';

import { Debug } from './modules/debug';

export const App: FC = () => {
  return (
    <div
      style={{
        position: 'relative'
      }}
    >
      <Debug />
    </div>
  );
};
