import React, {
  Dispatch,
  FC,
  SetStateAction,
  createContext,
  useState
} from 'react';
import ReactDOM from 'react-dom';

export interface DebugState {
  stepTime: number;
  collisionDetectionTime: number;
  broadPhaseTime: number;
  narrowPhaseTime: number;
  islandConstructionTime: number;
}

import { App } from './app';

const initialDebugState: DebugState = {
  stepTime: 0,
  collisionDetectionTime: 0,
  broadPhaseTime: 0,
  narrowPhaseTime: 0,
  islandConstructionTime: 0
};
export const DebugContext = createContext<DebugState>(initialDebugState);

async function main() {
  const shrm = await import('../src-wasm/pkg');
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
}

main();
