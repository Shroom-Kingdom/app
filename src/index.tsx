import React, {
  Dispatch,
  FC,
  SetStateAction,
  createContext,
  useState
} from 'react';
import ReactDOM from 'react-dom';
import { detect } from 'detect-browser';

export interface DebugState {
  stepTime: number;
  collisionDetectionTime: number;
  broadPhaseTime: number;
  narrowPhaseTime: number;
  islandConstructionTime: number;
}

import { App } from './app';
import { WebGL2FixHint, checkWebGL2 } from './webgl2';

const hasWebGL2Support = checkWebGL2();
const browser = detect();
export type Browser = typeof browser;

const initialDebugState: DebugState = {
  stepTime: 0,
  collisionDetectionTime: 0,
  broadPhaseTime: 0,
  narrowPhaseTime: 0,
  islandConstructionTime: 0
};
export const DebugContext = createContext<DebugState>(initialDebugState);

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
