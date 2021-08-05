import React, { FC, useState } from 'react';

export const App: FC = () => {
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
    </div>
  );
};
