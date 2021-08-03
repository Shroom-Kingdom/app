import React from 'react';
import ReactDOM from 'react-dom';

import { App } from './app';

export let Shrm: typeof import('../src-wasm/pkg');

(async () => {
  Shrm = await import('../src-wasm/pkg');
  Shrm.setupPanicHook();

  ReactDOM.render(<App />, document.getElementById('root'));
})();
