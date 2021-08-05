import React, { FC, useState } from 'react';

import { DebugState } from '.';

export const App: FC<{ debugState: DebugState }> = ({ debugState }) => {
  const [count, setCount] = useState<number>(0);
  return (
    <div
      style={{
        position: 'relative'
      }}
    >
      Hello WASM {count}
      <button
        onClick={() => {
          setCount(count + 1);
        }}
      >
        Increase
      </button>
      <div>Total: {debugState.stepTime}</div>
    </div>
  );
};
