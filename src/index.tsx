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
}

import { App } from './app';

async function main() {
  const shrm = await import('../src-wasm/pkg');
  shrm.setupPanicHook();

  let debugState: DebugState;
  let setDebugState: Dispatch<SetStateAction<DebugState>>;

  const WasmApp: FC = () => {
    const initialDebugState: DebugState = { stepTime: 0 };
    [debugState, setDebugState] = useState<DebugState>(initialDebugState);
    const DebugContext = createContext<DebugState>(initialDebugState);
    return (
      <React.StrictMode>
        <DebugContext.Provider value={debugState}>
          <App debugState={debugState} />
        </DebugContext.Provider>
      </React.StrictMode>
    );
  };

  ReactDOM.render(<WasmApp />, document.getElementById('root'));

  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  shrm.main(debugState!, setDebugState!);
}

main();
