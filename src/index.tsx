import React from 'react';
import ReactDOM from 'react-dom';

// import init, { hello } from '../src-wasm/pkg';

import { App } from './app';

export let Shrm: typeof import('../src-wasm/pkg');

(async () => {
  Shrm = await import('../src-wasm/pkg');
  Shrm.setupPanicHook();

  // await init();
  // Shrm = await import('../src-wasm/pkg');
  // Shrm = (await Shrm.default()) as any;
  // console.log('DEFAULT', await Shrm.default());
  // await (Shrm.default as any).load();
  // console.log('SHRM', Shrm);
  // await Shrm.setupPanicHook();
  console.log('FROM WASM', Shrm.hello());

  ReactDOM.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
    document.getElementById('root')
  );
})();
