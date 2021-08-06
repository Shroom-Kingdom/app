import React, { Dispatch, FC, SetStateAction, useState } from 'react';
import ReactDOM from 'react-dom';
import * as _ from 'styled-jsx'; // eslint-disable-line @typescript-eslint/no-unused-vars
import { detect } from 'detect-browser';

import { App } from './app';
import { WebGL2FixHint, checkWebGL2 } from './modules/webgl2';
import { DebugContext, DebugState, initialDebugState } from './modules/debug';

const hasWebGL2Support = checkWebGL2();
const browser = detect();
export type Browser = typeof browser;

async function main() {
  if (hasWebGL2Support) {
    const shrm = await import('../pkg');
    shrm.setupPanicHook();

    let debugState: DebugState;
    let setDebugState: Dispatch<SetStateAction<DebugState>>;

    const WasmApp: FC = () => {
      [debugState, setDebugState] = useState<DebugState>(initialDebugState);
      return (
        <React.StrictMode>
          <DebugContext.Provider value={debugState}>
            <App />
          </DebugContext.Provider>
        </React.StrictMode>
      );
    };

    ReactDOM.render(<WasmApp />, document.getElementById('root'));

    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    shrm.main(debugState!, setDebugState!);
  } else {
    const NoWebGL2App: FC = () => (
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          flexDirection: 'column',
          fontSize: '2rem',
          fontWeight: 500,
          textAlign: 'center'
        }}
      >
        <span>Your browser does not support WebGL2!</span>
        <WebGL2FixHint browser={browser} />
      </div>
    );

    ReactDOM.render(<NoWebGL2App />, document.getElementById('root'));
  }
}

main();
