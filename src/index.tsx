import React from 'react';
import ReactDOM from 'react-dom';

import { App } from './app';

async function main() {
  const shrm = await import('../src-wasm/pkg');
  shrm.setupPanicHook();
  shrm.main();
}

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);

main();
